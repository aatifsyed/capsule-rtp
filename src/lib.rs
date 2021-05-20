use anyhow::Result;
use capsule::packets::{ip::IpPacket, Internal, Packet, Udp};
use capsule::{
    packets::types::{u16be, u32be},
    SizeOf,
};
use std::default::Default;
use std::ptr::NonNull;

#[derive(Clone, Copy, Debug, Default, SizeOf)]
#[repr(C, packed)]
struct RtpHeader {
    version_padding_extension_csrc_count: u8,
    marker_payload_type: u8,
    sequence_number: u16be,
    timestamp: u32be,
    ssrc: u32be,
}

pub struct Rtp<P: IpPacket> {
    envelope: Udp<P>,
    header: NonNull<RtpHeader>,
    offset: usize,
}

impl<P: IpPacket> Rtp<P> {
    fn header(&self) -> &RtpHeader {
        unsafe { self.header.as_ref() }
    }

    fn header_mut(&mut self) -> &mut RtpHeader {
        unsafe { self.header.as_mut() }
    }

    pub fn csrc_count(&self) -> u8 {
        let mask = 0b0000_1111;
        let csrc_count = self.header().version_padding_extension_csrc_count & mask;
        csrc_count // endianness?
    }
}

impl<P: IpPacket> Packet for Rtp<P> {
    type Envelope = Udp<P>;

    fn envelope(&self) -> &Self::Envelope {
        &self.envelope
    }

    fn envelope_mut(&mut self) -> &mut Self::Envelope {
        &mut self.envelope
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn header_len(&self) -> usize {
        let minimum = RtpHeader::size_of();
        let csrcs = self.csrc_count() as usize * std::mem::size_of::<u32be>();
        minimum + csrcs
    }

    unsafe fn clone(&self, internal: Internal) -> Self {
        Rtp::<P> {
            envelope: self.envelope.clone(internal),
            header: self.header,
            offset: self.offset,
        }
    }

    fn try_parse(envelope: Self::Envelope, _internal: Internal) -> Result<Self>
    where
        Self: Sized,
    {
        let mbuf = envelope.mbuf();
        let offset = envelope.payload_offset();
        let header: NonNull<RtpHeader> = mbuf.read_data(offset)?;
        Ok(Rtp {
            envelope,
            header,
            offset,
        })
    }

    fn try_push(mut envelope: Self::Envelope, _internal: Internal) -> Result<Self>
    where
        Self: Sized,
    {
        let offset = envelope.payload_offset();
        let mbuf = envelope.mbuf_mut();

        mbuf.extend(offset, RtpHeader::size_of())?;
        let header = mbuf.write_data(offset, &RtpHeader::default())?;

        Ok(Rtp {
            envelope,
            header,
            offset,
        })
    }

    fn deparse(self) -> Self::Envelope
    where
        Self: Sized,
    {
        self.envelope
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csrc_count() {
        let mask: u8 = 0b_0000_1111;
        let inpt: u8 = 0b_1111_1000;
        let byte = inpt & mask;
        assert!(byte.to_be() == 8);
    }
}
