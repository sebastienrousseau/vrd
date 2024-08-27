#[cfg(test)]
mod tests {
    use vrd::mersenne_twister::{
        MersenneTwisterConfig, MersenneTwisterParams,
    };

    #[test]
    fn test_new_custom() {
        let params = MersenneTwisterParams {
            matrix_a: 0x9908b0df,
            upper_mask: 0x80000000,
            lower_mask: 0x7fffffff,
            tempering_mask_b: 0x9d2c5680,
            tempering_mask_c: 0xefc60000,
        };

        let config_result =
            MersenneTwisterConfig::<624, 397>::new_custom(params);
        assert!(config_result.is_ok());

        let config = config_result.unwrap();
        assert_eq!(config.params.matrix_a, 0x9908b0df);
        assert_eq!(config.params.upper_mask, 0x80000000);
        assert_eq!(config.params.lower_mask, 0x7fffffff);
        assert_eq!(config.params.tempering_mask_b, 0x9d2c5680);
        assert_eq!(config.params.tempering_mask_c, 0xefc60000);
    }

    #[test]
    #[should_panic(expected = "n must be at least 1")]
    fn test_new_custom_invalid_n() {
        let params = MersenneTwisterParams::default();
        let config_result =
            MersenneTwisterConfig::<0, 397>::new_custom(params);
        config_result.unwrap();
    }

    #[test]
    #[should_panic(expected = "m must be at least 1 and less than n")]
    fn test_new_custom_invalid_m() {
        let params = MersenneTwisterParams::default();
        let config_result =
            MersenneTwisterConfig::<624, 0>::new_custom(params);
        config_result.unwrap();
    }

    #[test]
    #[should_panic(expected = "matrix_a must have its highest bit set")]
    fn test_new_custom_invalid_matrix_a() {
        let params = MersenneTwisterParams {
            matrix_a: 0x7fffffff, // Invalid value
            upper_mask: 0x80000000,
            lower_mask: 0x7fffffff,
            tempering_mask_b: 0x9d2c5680,
            tempering_mask_c: 0xefc60000,
        };
        let config_result =
            MersenneTwisterConfig::<624, 397>::new_custom(params);
        config_result.unwrap();
    }

    #[test]
    #[should_panic(expected = "tempering_mask_b must be 0x9d2c5680")]
    fn test_new_custom_invalid_tempering_mask_b() {
        let params = MersenneTwisterParams {
            matrix_a: 0x9908b0df,
            upper_mask: 0x80000000,
            lower_mask: 0x7fffffff,
            tempering_mask_b: 0xffffffff, // Invalid value
            tempering_mask_c: 0xefc60000,
        };
        let config_result =
            MersenneTwisterConfig::<624, 397>::new_custom(params);
        config_result.unwrap();
    }

    #[test]
    #[should_panic(expected = "upper_mask must be 0x80000000")]
    fn test_new_custom_invalid_upper_mask() {
        let params = MersenneTwisterParams {
            matrix_a: 0x9908b0df,
            upper_mask: 0xffffffff, // Invalid value
            lower_mask: 0x7fffffff,
            tempering_mask_b: 0x9d2c5680,
            tempering_mask_c: 0xefc60000,
        };
        let config_result =
            MersenneTwisterConfig::<624, 397>::new_custom(params);
        config_result.unwrap();
    }

    #[test]
    #[should_panic(expected = "lower_mask must be 0x7fffffff")]
    fn test_new_custom_invalid_lower_mask() {
        let params = MersenneTwisterParams {
            matrix_a: 0x9908b0df,
            upper_mask: 0x80000000,
            lower_mask: 0xffffffff, // Invalid value
            tempering_mask_b: 0x9d2c5680,
            tempering_mask_c: 0xefc60000,
        };
        let config_result =
            MersenneTwisterConfig::<624, 397>::new_custom(params);
        config_result.unwrap();
    }

    #[test]
    #[should_panic(expected = "tempering_mask_c must be 0xefc60000")]
    fn test_new_custom_invalid_tempering_mask_c() {
        let params = MersenneTwisterParams {
            matrix_a: 0x9908b0df,
            upper_mask: 0x80000000,
            lower_mask: 0x7fffffff,
            tempering_mask_b: 0x9d2c5680,
            tempering_mask_c: 0xffffffff, // Invalid value
        };
        let config_result =
            MersenneTwisterConfig::<624, 397>::new_custom(params);
        config_result.unwrap();
    }

    #[test]
    fn test_set_config() {
        let mut config = MersenneTwisterConfig::<624, 397>::default();
        let params = MersenneTwisterParams::default();
        let set_result = config.set_config(params);

        assert!(set_result.is_ok());

        assert_eq!(config.params.matrix_a, 0x9908b0df);
        assert_eq!(config.params.upper_mask, 0x80000000);
        assert_eq!(config.params.lower_mask, 0x7fffffff);
        assert_eq!(config.params.tempering_mask_b, 0x9d2c5680);
        assert_eq!(config.params.tempering_mask_c, 0xefc60000);
    }

    #[test]
    fn test_new() {
        let config = MersenneTwisterConfig::<624, 397>::new().unwrap();

        assert_eq!(config.params.matrix_a, 0x9908b0df);
        assert_eq!(config.params.upper_mask, 0x80000000);
        assert_eq!(config.params.lower_mask, 0x7fffffff);
        assert_eq!(config.params.tempering_mask_b, 0x9d2c5680);
        assert_eq!(config.params.tempering_mask_c, 0xefc60000);
    }

    #[test]
    fn test_default() {
        let config = MersenneTwisterConfig::<624, 397>::default();
        assert_eq!(config.params.matrix_a, 0x9908b0df);
        assert_eq!(config.params.upper_mask, 0x80000000);
        assert_eq!(config.params.lower_mask, 0x7fffffff);
        assert_eq!(config.params.tempering_mask_b, 0x9d2c5680);
        assert_eq!(config.params.tempering_mask_c, 0xefc60000);
    }

    #[test]
    fn test_display() {
        let config = MersenneTwisterConfig::<624, 397>::new().unwrap();
        let expected = "MersenneTwisterConfig { params: MersenneTwisterParams { matrix_a: 0x9908b0df, upper_mask: 0x80000000, lower_mask: 0x7fffffff, tempering_mask_b: 0x9d2c5680, tempering_mask_c: 0xefc60000 } }";

        assert_eq!(format!("{}", config), expected);
    }

    #[test]
    fn test_validate_valid() {
        let params = MersenneTwisterParams::default();
        let validation_result =
            MersenneTwisterConfig::<624, 397>::validate(&params);
        assert!(validation_result.is_ok());
    }

    #[test]
    #[should_panic(expected = "n must be at least 1")]
    fn test_validate_invalid_n() {
        let params = MersenneTwisterParams::default();
        let validation_result =
            MersenneTwisterConfig::<0, 397>::validate(&params);
        validation_result.unwrap();
    }

    #[test]
    #[should_panic(expected = "m must be at least 1 and less than n")]
    fn test_validate_invalid_m() {
        let params = MersenneTwisterParams::default();
        let validation_result =
            MersenneTwisterConfig::<624, 0>::validate(&params);
        validation_result.unwrap();
    }

    #[test]
    #[should_panic(expected = "matrix_a must have its highest bit set")]
    fn test_validate_invalid_matrix_a() {
        let params = MersenneTwisterParams {
            matrix_a: 0x7fffffff, // Invalid value
            upper_mask: 0x80000000,
            lower_mask: 0x7fffffff,
            tempering_mask_b: 0x9d2c5680,
            tempering_mask_c: 0xefc60000,
        };
        let validation_result =
            MersenneTwisterConfig::<624, 397>::validate(&params);
        validation_result.unwrap();
    }

    #[test]
    #[should_panic(expected = "tempering_mask_b must be 0x9d2c5680")]
    fn test_validate_invalid_tempering_mask_b() {
        let params = MersenneTwisterParams {
            matrix_a: 0x9908b0df,
            upper_mask: 0x80000000,
            lower_mask: 0x7fffffff,
            tempering_mask_b: 0xffffffff, // Invalid value
            tempering_mask_c: 0xefc60000,
        };
        let validation_result =
            MersenneTwisterConfig::<624, 397>::validate(&params);
        validation_result.unwrap();
    }

    #[test]
    #[should_panic(expected = "upper_mask must be 0x80000000")]
    fn test_validate_invalid_upper_mask() {
        let params = MersenneTwisterParams {
            matrix_a: 0x9908b0df,
            upper_mask: 0xffffffff, // Invalid value
            lower_mask: 0x7fffffff,
            tempering_mask_b: 0x9d2c5680,
            tempering_mask_c: 0xefc60000,
        };
        let validation_result =
            MersenneTwisterConfig::<624, 397>::validate(&params);
        validation_result.unwrap();
    }

    #[test]
    #[should_panic(expected = "lower_mask must be 0x7fffffff")]
    fn test_validate_invalid_lower_mask() {
        let params = MersenneTwisterParams {
            matrix_a: 0x9908b0df,
            upper_mask: 0x80000000,
            lower_mask: 0xffffffff, // Invalid value
            tempering_mask_b: 0x9d2c5680,
            tempering_mask_c: 0xefc60000,
        };
        let validation_result =
            MersenneTwisterConfig::<624, 397>::validate(&params);
        validation_result.unwrap();
    }

    #[test]
    #[should_panic(expected = "tempering_mask_c must be 0xefc60000")]
    fn test_validate_invalid_tempering_mask_c() {
        let params = MersenneTwisterParams {
            matrix_a: 0x9908b0df,
            upper_mask: 0x80000000,
            lower_mask: 0x7fffffff,
            tempering_mask_b: 0x9d2c5680,
            tempering_mask_c: 0xffffffff, // Invalid value
        };
        let validation_result =
            MersenneTwisterConfig::<624, 397>::validate(&params);
        validation_result.unwrap();
    }
}
