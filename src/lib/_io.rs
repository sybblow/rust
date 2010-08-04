type buf_reader = unsafe obj {
  fn read() -> vec[u8];
};

type buf_writer = unsafe obj {
  fn write(vec[u8] v);
};

fn default_bufsz() -> uint {
  ret 4096u;
}

fn new_buf() -> vec[u8] {
  ret _vec.alloc[u8](default_bufsz());
}

fn new_buf_reader(str path) -> buf_reader {

  unsafe obj fd_buf_reader(int fd, mutable vec[u8] buf) {

    fn read() -> vec[u8] {

      // Ensure our buf is singly-referenced.
      if (_vec.rustrt.refcount[u8](buf) != 1u) {
        buf = new_buf();
      }

      auto len = _vec.len[u8](buf);
      auto vbuf = _vec.buf[u8](buf);
      auto count = os.libc.read(fd, vbuf, len);

      if (count < 0) {
        log "error filling buffer";
        log sys.rustrt.last_os_error();
        fail;
      } else {
        ret buf;
      }
    }

    drop {
      os.libc.close(fd);
    }
  }

  auto fd = os.libc.open(_str.buf(path), 0);
  if (fd < 0) {
    log "error opening file for reading";
    log sys.rustrt.last_os_error();
    fail;
  }
  ret fd_buf_reader(fd, new_buf());
}

fn new_buf_writer(str path) -> buf_writer {

  unsafe obj fd_buf_writer(int fd) {

    fn write(vec[u8] v) {
      auto len = _vec.len[u8](v);
      auto count = 0u;
      auto vbuf;
      while (count < len) {
        vbuf = _vec.buf_off[u8](v, count);
        auto nout = os.libc.write(fd, vbuf, len);
        if (nout < 0) {
          log "error dumping buffer";
          log sys.rustrt.last_os_error();
          fail;
        }
        count += nout as uint;
      }
    }

    drop {
      os.libc.close(fd);
    }
  }

  auto fd = os.libc.open(_str.buf(path), 0);
  if (fd < 0) {
    log "error opening file for writing";
    log sys.rustrt.last_os_error();
    fail;
  }
  ret fd_buf_writer(fd);
}
