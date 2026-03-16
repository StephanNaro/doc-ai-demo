// SPDX-License-Identifier: GPL-3.0-or-later

/// All supported document categories.
/// Keep this list exhaustive — every new category must be added here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    Invoices,
    EmploymentContracts,
    CustomerSupport,
    KnowledgeBase,
}

impl Category {
    /// Folder path relative to project root
    pub fn folder_path(&self) -> &'static str {
        match self {
            Category::Invoices             => "data/invoices",
            Category::EmploymentContracts  => "data/employment-contracts",
            Category::CustomerSupport      => "data/customer-support",
            Category::KnowledgeBase        => "data/knowledge-base",
        }
    }

    /// Human-readable name for UI/logs
    pub fn display_name(&self) -> &'static str {
        match self {
            Category::Invoices             => "Invoices",
            Category::EmploymentContracts  => "Employment Contracts",
            Category::CustomerSupport      => "Customer Support",
            Category::KnowledgeBase        => "Knowledge Base",
        }
    }

    /// Short identifier used in API requests / URLs
    pub fn api_value(&self) -> &'static str {
        match self {
            Category::Invoices             => "invoices",
            Category::EmploymentContracts  => "contracts",
            Category::CustomerSupport      => "support",
            Category::KnowledgeBase        => "knowledge",
        }
    }

    /// All valid string values that map to this category
    pub fn aliases(&self) -> &'static [&'static str] {
        match self {
            Category::Invoices => &["invoices", "invoice", "invoicing"],
            Category::EmploymentContracts => &["contracts", "employment-contracts", "contract", "employment"],
            Category::CustomerSupport => &["support", "customer-support", "tickets", "support-tickets"],
            Category::KnowledgeBase => &["knowledge", "knowledge-base", "kb", "policies", "faq"],
        }
    }

    /// Instructions to the AI tailored for each system role
    pub fn ai_instruction(&self) -> &'static str {
        match self {
            Category::Invoices =>
                "You are a precise invoice processor. Extract vendor, amounts (subtotal, VAT, total due), due date, invoice number, and payment terms exactly as written. Use keys like 'vendor', 'subtotal', 'vat', 'total_due', 'due_date', 'invoice_number'.",

            Category::EmploymentContracts => 
                "You are an expert employment contract reviewer. Focus on clauses, notice periods, leave entitlement, salary, non-compete, confidentiality, probation, remote work. Use keys like 'notice_period', 'annual_leave', 'salary', 'probation', 'non_compete'.",

            Category::CustomerSupport => 
                "You are a customer support analyst. Summarize the issue, customer details, requested action, severity, and suggest next steps. Use keys like 'issue_summary', 'customer', 'requested_action', 'suggested_reply'.",

            Category::KnowledgeBase => 
                "You are a company policy and internal knowledge assistant. Answer clearly and directly. Use keys like 'policy_answer', 'leave_days', 'remote_work_rules'. Quote sections or rules verbatim when relevant.",
        }
    }

    /// Try to parse a string (from API request) into a Category
    pub fn from_api_value(s: &str) -> Option<Self> {
        let lower = s.to_lowercase();
        for variant in [
            Category::Invoices,
            Category::EmploymentContracts,
            Category::CustomerSupport,
            Category::KnowledgeBase,
        ] {
            if variant.aliases().iter().any(|&a| a == lower) {
                return Some(variant);
            }
        }
        None
    }

    /// Returns a comma-separated (with 'or' before last) string of all valid api values
    pub fn all_api_values_human() -> String {
        let vals: Vec<&str> = ALL_CATEGORIES.iter().map(|c| c.api_value()).collect();
        if vals.is_empty() {
            return "none".to_string();
        }
        if vals.len() == 1 {
            return vals[0].to_string();
        }
        let joined = vals[..vals.len()-1].join(", ");
        format!("{}, or {}", joined, vals.last().unwrap())
    }

    /// Default/fallback category
    pub const DEFAULT: Self = Category::Invoices;
}

/// All known categories in a const array (useful for iteration, dropdown generation, etc.)
pub const ALL_CATEGORIES: &[Category] = &[
    Category::Invoices,
    Category::EmploymentContracts,
    Category::CustomerSupport,
    Category::KnowledgeBase,
];