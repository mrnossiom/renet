# Renet

**Goal 1**: be able to make an HTTP request with Ethernet access as the sole base.

- [ ] Ethernet ([IEEE 802.3][ethernet-ieee-std])
  There are multiple types of [Ethernet frames][wiki-ethernet-frame] that can coexist on a network.
  Ethernet II frames, as described in *IEEE 802.3*, are the most common.
- [ ] IPv4 ([RFC 791])
- [ ] IPv6 ([RFC 8200])
- [ ] TCP ([RFC 9293])
- [ ] DNS ([RFC 1034], [RFC 1035])
- [ ] ARP ([RFC 826])
- [ ] DHCP ([RFC 2131])

[wiki-ethernet-frame]: https://en.wikipedia.org/wiki/Ethernet_frame
[ethernet-ieee-std]: https://standards.ieee.org/ieee/802.3/10422/
[RFC 791]: https://www.rfc-editor.org/info/rfc791/
[RFC 826]: https://www.rfc-editor.org/info/rfc826/
[RFC 1034]: https://www.rfc-editor.org/info/rfc1034/
[RFC 1035]: https://www.rfc-editor.org/info/rfc1035/
[RFC 2131]: https://www.rfc-editor.org/info/rfc2131/
[RFC 8200]: https://www.rfc-editor.org/info/rfc8200/
[RFC 9293]: https://www.rfc-editor.org/info/rfc9293/

## References

- [Create virtual network interfaces on Linux](https://www.thelinuxvault.net/blog/linux-create-virtual-network-interface/)
- [`ip tuntap` examples](https://baturin.org/docs/iproute2/#ip-tuntap)
