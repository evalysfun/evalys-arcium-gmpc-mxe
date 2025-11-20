use arcis_imports::*;

/// Confidential multi-user analytics
/// 
/// Aggregates insights across multiple users without exposing individual behavior.
/// Uses gMPC to compute aggregated metrics while keeping all user data encrypted.
#[encrypted]
mod circuits {
    use arcis_imports::*;

    /// Anonymized user profile (encrypted)
    pub struct UserProfile {
        pub profile_id_hash: u64,         // Hashed profile ID (not reversible)
        pub avg_hold_time: u32,           // Average hold time in seconds
        pub win_rate: u16,                 // Win rate (0-10000, basis points)
        pub preferred_size_range_min: u64, // Min preferred size
        pub preferred_size_range_max: u64, // Max preferred size
        pub risk_tolerance: u8,            // 0=low, 1=normal, 2=high
    }

    /// Aggregation type (plaintext)
    pub enum AggregationType {
        CurveInsights = 0,
        MarketSentiment = 1,
        PatternDetection = 2,
    }

    /// Aggregated metrics output (encrypted)
    pub struct AggregatedMetrics {
        pub avg_hold_time: u32,            // Average hold time across users
        pub avg_win_rate: u16,             // Average win rate
        pub common_size_range_min: u64,    // Common min size preference
        pub common_size_range_max: u64,    // Common max size preference
        pub risk_distribution: [u16; 3],   // Distribution: [low, normal, high]
        pub confidence_score: u16,          // Confidence in aggregation (0-10000)
        pub sample_size: u32,              // Number of users in aggregation
    }

    #[instruction]
    pub fn confidential_multi_user_analytics(
        profiles: Vec<Enc<Shared, UserProfile>>,
        aggregation_type: AggregationType,
    ) -> Enc<Shared, AggregatedMetrics> {
        // Decrypt all profiles (inside MPC, never exposed)
        let decrypted_profiles: Vec<_> = profiles.iter()
            .map(|p| p.to_arcis())
            .collect();

        let sample_size = decrypted_profiles.len() as u32;

        if sample_size == 0 {
            // Return empty metrics if no profiles
            return profiles[0].owner.from_arcis(AggregatedMetrics {
                avg_hold_time: 0,
                avg_win_rate: 0,
                common_size_range_min: 0,
                common_size_range_max: 0,
                risk_distribution: [0, 0, 0],
                confidence_score: 0,
                sample_size: 0,
            });
        }

        // Compute aggregated metrics (all inside MPC)
        let total_hold_time: u64 = decrypted_profiles.iter()
            .map(|p| p.avg_hold_time as u64)
            .sum();
        let avg_hold_time = (total_hold_time / sample_size as u64) as u32;

        let total_win_rate: u64 = decrypted_profiles.iter()
            .map(|p| p.win_rate as u64)
            .sum();
        let avg_win_rate = (total_win_rate / sample_size as u64) as u16;

        // Find common size range (intersection of all ranges)
        let common_min = decrypted_profiles.iter()
            .map(|p| p.preferred_size_range_min)
            .max()
            .unwrap_or(0);
        let common_max = decrypted_profiles.iter()
            .map(|p| p.preferred_size_range_max)
            .min()
            .unwrap_or(0);

        // Risk distribution
        let mut risk_dist = [0u16; 3];
        for profile in &decrypted_profiles {
            match profile.risk_tolerance {
                0 => risk_dist[0] += 1,
                1 => risk_dist[1] += 1,
                _ => risk_dist[2] += 1,
            }
        }

        // Confidence score based on sample size and consistency
        let consistency = if common_max > common_min {
            // Valid range intersection
            8000u16
        } else {
            // No valid intersection
            3000u16
        };

        let sample_confidence = if sample_size >= 10 {
            10000u16
        } else if sample_size >= 5 {
            7000u16
        } else {
            4000u16
        };

        let confidence_score = (consistency + sample_confidence) / 2;

        let metrics = AggregatedMetrics {
            avg_hold_time,
            avg_win_rate,
            common_size_range_min: common_min,
            common_size_range_max: common_max,
            risk_distribution: risk_dist,
            confidence_score,
            sample_size,
        };

        // Return encrypted result (owned by first profile's owner)
        profiles[0].owner.from_arcis(metrics)
    }
}

