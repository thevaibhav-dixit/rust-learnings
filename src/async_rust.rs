pub async fn non_blocking() {
    let task1 = async {
        println!("Task 1 is running");
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        println!("Task 1 completed");
    };

    let task2 = async {
        println!("Task 2 is running");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("Task 2 completed");
    };

    // this join is non-blocking that means even though task1 start first it will not block task2
    // from starting. task1 yieds control back to the main thread which executes task2. when task2
    // is waiting for 1 second control is yielded back to the main thread which then does nothing.
    // when task2 wakes up via epoll sys call it will be scheduled to run again and is future is in
    // ready state and control is given back to the main thread which waits for 1 second more for
    // task 1 to be in Ready state.
    tokio::join!(task1, task2);
}
