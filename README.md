# Adding RTP packets to capsule
[Capsule](https://github.com/capsule-rs/capsule) is a rusty DPDK abstraction. 
I'd be interested in using that to generate RTP, so I'm experimenting with that here.

# Docker
I didn't want to fuss around with Vagrant etc, so this repository also _contains_ configuration files for working _inside_ a container. 
For tests etc, I'm envisioning using a veth pair, and plugging into an application, again staying inside the container. 

# Ideas
- Use `rtp-rs` instead of rolling my own RTP
