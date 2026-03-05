//! Shared data types used across multiple DataXLR8 MCP servers.
//!
//! These types are produced by enrichment-mcp and consumed by
//! crm-mcp, email-mcp, sales-mcp, and other downstream MCPs.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Enriched person data from one or more providers.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonData {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub title: Option<String>,
    pub company: Option<String>,
    pub linkedin_url: Option<String>,
    pub github_url: Option<String>,
    pub twitter_url: Option<String>,
    pub phone: Option<String>,
    pub location: Option<String>,
    /// Confidence score from 0.0 to 1.0.
    pub confidence: f64,
    /// Which provider produced this data.
    pub source: String,
}

/// Enriched company data from one or more providers.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompanyData {
    pub name: Option<String>,
    pub domain: Option<String>,
    pub description: Option<String>,
    pub industry: Option<String>,
    /// Company size bucket: "1-10", "11-50", "51-200", etc.
    pub size: Option<String>,
    pub tech_stack: Vec<String>,
    pub social_profiles: HashMap<String, String>,
    pub location: Option<String>,
    pub founded_year: Option<i32>,
    pub logo_url: Option<String>,
    /// Confidence score from 0.0 to 1.0.
    pub confidence: f64,
    /// Which provider produced this data.
    pub source: String,
}

/// Result of verifying an email address.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailVerification {
    pub email: String,
    pub deliverable: bool,
    pub catch_all: bool,
    pub disposable: bool,
    pub mx_found: bool,
    pub smtp_verified: bool,
    /// Confidence score from 0.0 to 1.0.
    pub confidence: f64,
    /// Which provider performed the verification.
    pub source: String,
}

/// A candidate email address with pattern and verification status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailCandidate {
    pub email: String,
    /// Pattern used: "first.last", "flast", "first", etc.
    pub pattern: String,
    pub verified: bool,
    /// Confidence score from 0.0 to 1.0.
    pub confidence: f64,
}
