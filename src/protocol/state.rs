#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
)]
pub enum RecoveryState {
    /// Request vừa được tạo
    Created,

    /// Request đã được chủ sở hữu ký
    Signed,

    /// Committee đang xử lý
    Pending,

    /// Committee đã chấp nhận
    Approved,

    /// Committee từ chối
    Rejected,

    /// Recovery hoàn tất
    Executed,
}

impl RecoveryState {

    /// Whether the request is in a terminal state.
    pub fn is_final(&self) -> bool {
        matches!(
            self,
            RecoveryState::Approved
                | RecoveryState::Rejected
                | RecoveryState::Executed
        )
    }

}