TAG =? v3.0.1

install:
	@ git clone https://github.com/new-kernel/novusk/
	@ cd novusk/ && git chckout $TAG
	@ mv novusk/include/novusk novuskinc/
