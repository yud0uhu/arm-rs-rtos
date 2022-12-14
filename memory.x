/* LM3S6965マイクロコントローラのメモリレイアウト */
/* 1K = 1 KiBi = 1024バイト */
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x00000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 64K
}