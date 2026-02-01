use super::FlashStore;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m::interrupt;

/// 存储的起始地址，FLASH的最后一页（1KB）
const STORE_START_ADDRESS: u32 = 0x0800FC00;
/// 存储数据的个数
// const STORE_COUNT: usize = 512;

/// 定义SRAM数组，
// Recell：为静态不可变的数组提供内部可变性，把编译期的可变性检查推迟到运行时
// Mutex：中断安全的互斥保护，避免主循环和中断同时访问数组
static STORE_DATA: Mutex<RefCell<[u16; 512]>> =
    Mutex::new(RefCell::new([0; 512]));
impl<'a> FlashStore<'a> {

    /// 在保证中断安全的前提下，向全局静态缓存数组STORE_DATA中指定位置的写数值
pub fn set_store(&self, i: usize, value: u16) {
    // 中断临界区：禁用所有可屏蔽中断，生成cs临界区令牌，将令牌传入Mutex解锁用以访问缓存数组
    // 并获取获取RefCell的排他性可写引用（保证同一时间只有一个写操作），通过可写引用访问数组
    // 的第i个元素，将其赋值为value
    cortex_m::interrupt::free(|cs| {
        STORE_DATA.borrow(cs).borrow_mut()[i] = value;
    });
}
    // 在保证中断安全的前提下，读取全局静态缓存数组STORE_DATA中指定位置的数值
pub fn get_store(&self, i: usize) -> u16 {
    cortex_m::interrupt::free(|cs| {
        // 获取RefCell的共享只读引用
        // Mutex::borrow ：凭临界区令牌解锁互斥锁，保证中断安全访问
        //RefCell::borrow：申请数组的只读权限，保证运行时无读写冲突
        STORE_DATA.borrow(cs).borrow()[i]
    })
}

    /// 定义参数存储模块初始化函数
pub fn init_store(&self) {
    interrupt::free(|cs| {
        //
        let mut store = STORE_DATA.borrow(cs).borrow_mut();

        // 判断是否第一次使用
        if FlashStore::flash_read_half_word(STORE_START_ADDRESS) != 0xA5A5 {
            // 擦除Flash目标页，写入前必须先擦除
            self.flash_erase_page(STORE_START_ADDRESS);
            // 写入初始化标志（标记 Flash 已初始化），避免重复擦除
            self.flash_program_half_word(STORE_START_ADDRESS, 0xA5A5);
            for i in 1..store.len() {
                let address = STORE_START_ADDRESS + i as u32 * 2;
                self.flash_program_half_word(address, 0x0000);
            }
        }
        // Flash → SRAM：将Flash中所有持久化的参数数据，读取并写入到SRAM缓存STORE_DATA中
        for i in 0..store.len() {
            let address = STORE_START_ADDRESS + i as u32 * 2;
            store[i] = FlashStore::flash_read_half_word(address);
        }
    });
}

    /// 定义参数存储模块保存数据到闪存函数
pub fn store_save(&self) {
    // 擦除指定页
    self.flash_erase_page(STORE_START_ADDRESS);
    // 中断临界区内，将 SRAM 缓存同步到 Flash
    interrupt::free(|cs| {
        // 获取 SRAM 缓存的只读引用
        let store = STORE_DATA.borrow(cs).borrow();
        // 包括第一个标志位，遍历所有缓存数据，半字写入
        for i in 0..store.len() {
            let address = STORE_START_ADDRESS + i as u32 * 2;
            self.flash_program_half_word(address, store[i]);
        }
    });
}
    /// 定义参数存储模块将所有有效数据清0函数
pub fn store_clear(&self) {
    interrupt::free(|cs| {
        // 获取获取 SRAM 缓存的可写引用
        let mut store = STORE_DATA.borrow(cs).borrow_mut();
        // 除了第一个标志位，遍历数组清空
        for i in 1..store.len() {
            store[i] = 0x0000;
        }
    });
    // 同步数据到Flash
    self.store_save();
}

}