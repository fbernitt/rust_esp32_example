#!/usr/bin/env bash

set -eu -o pipefail

_process_command_chain() {
  if [ $# -gt 0 ] ; then
    $0 "$@"
  fi
}

task_build() {
  docker run --rm -ti -v "$(pwd):/home/project" rust-esp-container quick-build

  _process_command_chain "$@"
}

task_flash() {
  esptool.py \
       --chip esp32 \
       --port /dev/cu.usbserial-0001 \
       --baud 115200 \
       --before default_reset \
       --after hard_reset \
       write_flash \
       -z \
       --flash_mode dio \
       --flash_freq 40m \
       --flash_size detect \
       0x1000 build/bootloader/bootloader.bin \
       0x10000 build/esp-app.bin
       # 0x8000 build/partitions_singleapp.bin
}

task_format() {
  rustfmt src/main.rs

  _process_command_chain "$@"
}

task_help() {
  cat <<EOF
Usage $0 COMMAND

Commands are:

  build                 Cross compile the app
  flash			Flash onto ESP32
  format		Format sources
EOF
}

CMD=${1:-}
shift || true

case "${CMD}" in
  build) task_build "$@" ;;
  flash) task_flash "$@" ;;
  format) task_format "$@" ;;
  *) task_help ;;
esac
