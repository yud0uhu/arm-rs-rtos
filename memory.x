/* LM3S6965マイクロコントローラのメモリレイアウト */
/* 1K = 1 KiBi = 1024バイト */
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH :  ORIGIN = 0x08000000, LENGTH = 1024K
  RAM : ORIGIN = 0x20000000, LENGTH = 128K
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM);