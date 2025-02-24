use anchor_lang::prelude::*;

declare_id!("7E7nvzfrapNcHYDeBjyHK7YnGo1w1PqSUNfeLPtodJxc");

#[program]
pub mod max_time_assert {
    use super::*;

    pub fn assert_max_timestamp(
        ctx: Context<AssertMaxTimestamp>,
        max_timestamp: u64,
    ) -> Result<()> {
        let clock = Clock::get()?;
        assert!(clock.unix_timestamp < max_timestamp as i64);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AssertMaxTimestamp {}
