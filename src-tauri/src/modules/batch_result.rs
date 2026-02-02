// 批量操作结果类型
use serde::{Deserialize, Serialize};

/// 批量操作单项结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchItemResult {
    pub profile_id: String,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl BatchItemResult {
    pub fn success(profile_id: String) -> Self {
        Self {
            profile_id,
            ok: true,
            error: None,
        }
    }

    pub fn failure(profile_id: String, error: String) -> Self {
        Self {
            profile_id,
            ok: false,
            error: Some(error),
        }
    }
}

/// 批量操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResult {
    pub results: Vec<BatchItemResult>,
    pub total: usize,
    pub success_count: usize,
    pub failure_count: usize,
}

impl BatchResult {
    pub fn from_results(results: Vec<BatchItemResult>) -> Self {
        let total = results.len();
        let success_count = results.iter().filter(|r| r.ok).count();
        let failure_count = total - success_count;

        Self {
            results,
            total,
            success_count,
            failure_count,
        }
    }

    /// 检查是否全部成功
    pub fn is_all_success(&self) -> bool {
        self.failure_count == 0
    }

    /// 检查是否全部失败
    pub fn is_all_failure(&self) -> bool {
        self.success_count == 0
    }

    /// 检查是否部分成功
    pub fn is_partial_success(&self) -> bool {
        self.success_count > 0 && self.failure_count > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_item_result_success() {
        let result = BatchItemResult::success("profile_1".to_string());
        assert!(result.ok);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_batch_item_result_failure() {
        let result = BatchItemResult::failure("profile_1".to_string(), "Error message".to_string());
        assert!(!result.ok);
        assert_eq!(result.error.as_ref().unwrap(), "Error message");
    }

    #[test]
    fn test_batch_result_all_success() {
        let results = vec![
            BatchItemResult::success("p1".to_string()),
            BatchItemResult::success("p2".to_string()),
        ];
        let batch = BatchResult::from_results(results);

        assert_eq!(batch.total, 2);
        assert_eq!(batch.success_count, 2);
        assert_eq!(batch.failure_count, 0);
        assert!(batch.is_all_success());
        assert!(!batch.is_all_failure());
        assert!(!batch.is_partial_success());
    }

    #[test]
    fn test_batch_result_partial_success() {
        let results = vec![
            BatchItemResult::success("p1".to_string()),
            BatchItemResult::failure("p2".to_string(), "Error".to_string()),
            BatchItemResult::success("p3".to_string()),
        ];
        let batch = BatchResult::from_results(results);

        assert_eq!(batch.total, 3);
        assert_eq!(batch.success_count, 2);
        assert_eq!(batch.failure_count, 1);
        assert!(!batch.is_all_success());
        assert!(!batch.is_all_failure());
        assert!(batch.is_partial_success());
    }

    #[test]
    fn test_batch_result_all_failure() {
        let results = vec![
            BatchItemResult::failure("p1".to_string(), "Error 1".to_string()),
            BatchItemResult::failure("p2".to_string(), "Error 2".to_string()),
        ];
        let batch = BatchResult::from_results(results);

        assert_eq!(batch.total, 2);
        assert_eq!(batch.success_count, 0);
        assert_eq!(batch.failure_count, 2);
        assert!(!batch.is_all_success());
        assert!(batch.is_all_failure());
        assert!(!batch.is_partial_success());
    }
}
