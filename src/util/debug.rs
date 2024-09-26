#[macro_export]
macro_rules! check_zero_warn {
    ($val:expr) => {
        {
          let val = $val;
          if num_traits::Zero::is_zero(&val) {
              log::warn!("Value '{}' is zero: {}", stringify!($val), $val);
          }
        }
    };
}
