FROM postgres:latest

# config de postgresql pour être en Français:
# cf https://github.com/docker-library/docs/blob/master/postgres/README.md#locale-customization
RUN localedef -i fr_FR -c -f UTF-8 -A /usr/share/locale/locale.alias fr_FR.UTF-8
ENV LANG=fr_FR.utf8