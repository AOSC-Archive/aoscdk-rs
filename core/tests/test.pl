use strict;

chroot("/tmp");
system("ls", "/");
chroot("../../../../../../");
system("ls", "/tmp");
