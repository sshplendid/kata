// lib.rs

/*
 * communicator
 * ├─client
 * └─network
 *   ├─server
 *   └─client
 */
pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
/*
 * communicator // (1)번: 루트 라이브러리로 올라가서 하위 라이브러리 접근, 라이브러리의 규모가 큰
 * 경우 복잡해질 수 있다.
 * ├─client    **여길 찾아가야 함
 * ├─test      // (2)번: 현재 위치에서 부모 모듈에 접근한 뒤 client 라이브러리에 접근
 * └─network
 *   ├─server
 *   └─client
 */
        ::client::connect();      // (1)번
        super::client::connect(); // (2)번
        assert_eq!(2 + 2, 4);
    }
}
