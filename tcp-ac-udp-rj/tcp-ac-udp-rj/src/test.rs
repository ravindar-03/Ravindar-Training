#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::{TcpStream, UdpSocket};
    use tokio::runtime::Runtime;
    use std::time::Duration;

    // Helper function to simulate environment setup
    async fn setup_xdp() -> Result<(), anyhow::Error> {
        let opt = Opt { iface: "eth0".to_string() };
        env_logger::init();

        #[cfg(debug_assertions)]
        let mut bpf = Bpf::load(include_bytes_aligned!(
            "../../target/bpfel-unknown-none/debug/tcp-ac-udp-rj"
        ))?;
        #[cfg(not(debug_assertions))]
        let mut bpf = Bpf::load(include_bytes_aligned!(
            "../../target/bpfel-unknown-none/release/tcp-ac-udp-rj"
        ))?;

        BpfLogger::init(&mut bpf)?;
        let program: &mut Xdp = bpf.program_mut("xdp_firewall")?.try_into()?;
        program.load()?;
        program.attach(&opt.iface, XdpFlags::default())
            .context("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE")?;

        Ok(())
    }

    #[tokio::test]
    async fn test_tcp_connection_acceptance() -> Result<(), anyhow::Error> {
        setup_xdp().await?;

        // Assume the server listens on port 12345
        let addr = "127.0.0.1:12345";
        let tcp_result = TcpStream::connect(addr).await;
        assert!(tcp_result.is_ok(), "TCP connection should be accepted");

        Ok(())
    }

    #[tokio::test]
    async fn test_udp_connection_rejection() -> Result<(), anyhow::Error> {
        setup_xdp().await?;

        // Assume the server listens on port 12345
        let addr = "127.0.0.1:12345";
        let udp_socket = UdpSocket::bind("127.0.0.1:0").await?;
        let udp_result = udp_socket.send_to(b"test", addr).await;

        // Since UDP rejection is usually silent, we can use a timeout to detect no response
        let mut buf = [0; 4];
        let recv_result = tokio::time::timeout(Duration::from_secs(2), udp_socket.recv_from(&mut buf)).await;

        assert!(udp_result.is_ok(), "UDP packet should be sent");
        assert!(recv_result.is_err(), "UDP response should be rejected or timed out");

        Ok(())
    }
}
