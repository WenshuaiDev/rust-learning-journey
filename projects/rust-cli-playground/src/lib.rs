// 为什么此处不能用 pub(crate) mod cli
// 因为lib与main是两个完全不同的crate，虽然他们同名
// pub（crate）只是说明了这个mod在本crate下可用，其他crate不可用
// 所以如果加上 pub(crate)限定，在main中就显示cli这个是private，不可用
pub mod cli;