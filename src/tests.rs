use crate::snake::Snake;

#[test]
fn test_develop() {
    let mut snake = Snake::new(5, 10, 0);

    let result = vec![
        vec![0],
        vec![0, 1],
        vec![0, 1, 2],
        vec![0, 1, 2, 3],
        vec![0, 1, 2, 3, 4],
        vec![1, 2, 3, 4, 5],
        vec![2, 3, 4, 5, 6],
        vec![3, 4, 5, 6, 7],
        vec![4, 5, 6, 7, 8],
        vec![5, 6, 7, 8, 9],
        vec![6, 7, 8, 9, 10],
        vec![7, 8, 9, 10],
        vec![8, 9, 10],
        vec![9, 10],
        vec![10],
        vec![],
    ];

    for item in result {
        snake.develop();
        assert_eq!(item, snake.current);
    }
}
