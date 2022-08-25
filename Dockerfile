FROM fedora:latest

RUN dnf update -y
RUN dnf install git xorg-x11-server-Xvfb procps-ng wget libjpeg-turbo-devel expat-devel mold 'dnf-command(builddep)' -y
RUN dnf builddep gtk4 -y

# build gtk4
RUN git clone https://gitlab.gnome.org/gnome/gtk.git --depth=1
WORKDIR gtk
RUN meson setup builddir --prefix=/usr -Dgtk_doc=false -Dintrospection=disabled -Dbuild-examples=false -Dbuild-tests=false -Ddemos=false
RUN meson install -C builddir
WORKDIR /
RUN rm -rf gtk

# setup mold as default linker
RUN ln -sf $(which mold) $(realpath /usr/bin/ld)

# build libadwaita
RUN git clone https://gitlab.gnome.org/GNOME/libadwaita.git -b 1.1.1
WORKDIR libadwaita
RUN meson setup builddir --prefix=/usr -Dintrospection=disabled -Dvapi=false -Dexamples=false -Dtests=false
RUN meson install -C builddir
WORKDIR /
RUN rm -rf libadwaita
