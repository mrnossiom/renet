_default:
    just --list

[linux]
create-tap0:
    ip tuntap add name tap0 mode tap user $USER
