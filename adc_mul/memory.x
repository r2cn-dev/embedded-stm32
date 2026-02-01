/* Linker script for the STM32F103C8T6 */

/*定义了芯片的程序存储区flash和数据存储区ARM，ORIGIN是链接脚本中定义物理内存起始地址的关键字*/
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K
  RAM : ORIGIN = 0x20000000, LENGTH = 20K
}