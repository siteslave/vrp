use crate::helpers::*;
use crate::json::problem::*;
use crate::json::solution::*;

#[test]
fn can_limit_one_job_by_shift_time() {
    let problem = Problem {
        id: "my_problem".to_string(),
        plan: Plan { jobs: vec![create_delivery_job("job1", vec![100., 0.])], relations: Option::None },
        fleet: Fleet {
            types: vec![VehicleType {
                id: "my_vehicle".to_string(),
                profile: "car".to_string(),
                costs: create_default_vehicle_costs(),
                places: create_default_vehicle_places(),
                capacity: vec![10],
                amount: 1,
                skills: None,
                limits: Some(VehicleLimits { max_distance: None, shift_time: Some(99.) }),
                vehicle_break: None,
            }],
        },
    };
    let matrix = Matrix {
        num_origins: 2,
        num_destinations: 2,
        travel_times: vec![1, 100, 100, 1],
        distances: vec![1, 1, 1, 1],
        error_codes: Option::None,
    };

    let solution = solve_with_metaheuristic(problem, vec![matrix]);

    assert_eq!(
        solution,
        Solution {
            problem_id: "my_problem".to_string(),
            statistic: Statistic {
                cost: 0.,
                distance: 0,
                duration: 0,
                times: Timing { driving: 0, serving: 0, waiting: 0, break_time: 0 },
            },
            tours: vec![],
            unassigned: vec![UnassignedJob {
                job_id: "job1".to_string(),
                reasons: vec![UnassignedJobReason {
                    code: 102,
                    description: "cannot be assigned due to shift time constraint of vehicle".to_string()
                }]
            }],
            extras: Extras { performance: vec![] },
        }
    );
}

#[test]
fn can_skip_job_from_multiple_because_of_shift_time() {
    let problem = Problem {
        id: "my_problem".to_string(),
        plan: Plan {
            jobs: vec![
                create_delivery_job_with_duration("job1", vec![1., 0.], 10.),
                create_delivery_job_with_duration("job2", vec![2., 0.], 10.),
                create_delivery_job_with_duration("job3", vec![3., 0.], 10.),
                create_delivery_job_with_duration("job4", vec![4., 0.], 10.),
                create_delivery_job_with_duration("job5", vec![5., 0.], 10.),
            ],
            relations: Option::None,
        },
        fleet: Fleet {
            types: vec![VehicleType {
                id: "my_vehicle".to_string(),
                profile: "car".to_string(),
                costs: create_default_vehicle_costs(),
                places: create_default_vehicle_places(),
                capacity: vec![10],
                amount: 1,
                skills: None,
                limits: Some(VehicleLimits { max_distance: None, shift_time: Some(40.) }),
                vehicle_break: None,
            }],
        },
    };
    let matrix = create_matrix(vec![
        0, 1, 2, 3, 4, 1, 1, 0, 1, 2, 3, 2, 2, 1, 0, 1, 2, 3, 3, 2, 1, 0, 1, 4, 4, 3, 2, 1, 0, 5, 1, 2, 3, 4, 5, 0,
    ]);

    let solution = solve_with_metaheuristic(problem, vec![matrix]);

    assert_eq!(
        solution,
        Solution {
            problem_id: "my_problem".to_string(),
            statistic: Statistic {
                cost: 52.,
                distance: 6,
                duration: 36,
                times: Timing { driving: 6, serving: 30, waiting: 0, break_time: 0 },
            },
            tours: vec![Tour {
                vehicle_id: "my_vehicle_1".to_string(),
                type_id: "my_vehicle".to_string(),
                stops: vec![
                    create_stop_with_activity(
                        "departure",
                        "departure",
                        (0., 0.),
                        3,
                        ("1970-01-01T00:00:00Z", "1970-01-01T00:00:00Z"),
                    ),
                    create_stop_with_activity(
                        "job3",
                        "delivery",
                        (3., 0.),
                        2,
                        ("1970-01-01T00:00:03Z", "1970-01-01T00:00:13Z"),
                    ),
                    create_stop_with_activity(
                        "job2",
                        "delivery",
                        (2., 0.),
                        1,
                        ("1970-01-01T00:00:14Z", "1970-01-01T00:00:24Z"),
                    ),
                    create_stop_with_activity(
                        "job1",
                        "delivery",
                        (1., 0.),
                        0,
                        ("1970-01-01T00:00:25Z", "1970-01-01T00:00:35Z"),
                    ),
                    create_stop_with_activity(
                        "arrival",
                        "arrival",
                        (0., 0.),
                        0,
                        ("1970-01-01T00:00:36Z", "1970-01-01T00:00:36Z"),
                    )
                ],
                statistic: Statistic {
                    cost: 52.,
                    distance: 6,
                    duration: 36,
                    times: Timing { driving: 6, serving: 30, waiting: 0, break_time: 0 },
                },
            }],
            unassigned: vec![
                UnassignedJob {
                    job_id: "job4".to_string(),
                    reasons: vec![UnassignedJobReason {
                        code: 102,
                        description: "cannot be assigned due to shift time constraint of vehicle".to_string()
                    }]
                },
                UnassignedJob {
                    job_id: "job5".to_string(),
                    reasons: vec![UnassignedJobReason {
                        code: 102,
                        description: "cannot be assigned due to shift time constraint of vehicle".to_string()
                    }]
                }
            ],
            extras: Extras { performance: vec![] },
        }
    );
}