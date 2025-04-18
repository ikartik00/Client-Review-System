#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct Review {
    pub reviewer: String,
    pub reviewee: String,
    pub rating: u32,
    pub comment: String,
}

#[contracttype]
pub enum ReviewBook {
    ReviewEntry(u64)
}

const REVIEW_COUNT: Symbol = symbol_short!("R_COUNT");

#[contract]
pub struct ClientReviewContract;

#[contractimpl]
impl ClientReviewContract {
    // Add a review
    pub fn add_review(env: Env, reviewer: String, reviewee: String, rating: u32, comment: String) -> u64 {
        let mut count: u64 = env.storage().instance().get(&REVIEW_COUNT).unwrap_or(0);
        count += 1;

        let review = Review {
            reviewer,
            reviewee,
            rating,
            comment,
        };

        env.storage().instance().set(&ReviewBook::ReviewEntry(count), &review);
        env.storage().instance().set(&REVIEW_COUNT, &count);
        env.storage().instance().extend_ttl(5000, 5000);

        count
    }

    // View a review by ID
    pub fn view_review(env: Env, review_id: u64) -> Review {
        env.storage().instance().get(&ReviewBook::ReviewEntry(review_id)).unwrap_or(Review {
            reviewer: String::from_str(&env, "Not_Found"),
            reviewee: String::from_str(&env, "Not_Found"),
            rating: 0,
            comment: String::from_str(&env, "No comment found"),
        })
    }

    // Count of total reviews
    pub fn total_reviews(env: Env) -> u64 {
        env.storage().instance().get(&REVIEW_COUNT).unwrap_or(0)
    }
}
