use std::iter;
use async_hid::AsyncHidRead;
use async_hid::{AsyncHidWrite, DeviceReaderWriter, HidBackend, HidResult};
use futures_lite::stream::StreamExt;

pub struct MSR605x {
    device: DeviceReaderWriter
}

impl MSR605x {
    pub async fn new() -> HidResult<Self> {
        // fix to eliminate panic
        let device = HidBackend::default().enumerate().await?
            .find(|info| info.matches(0x1, 0x0, 0x0801, 0x0003))
            .await.expect("Could not open card reader").open().await?;
        Ok(MSR605x { device })
    }

    pub async fn cmd(&mut self, cmd: &[u8]) -> HidResult<()> {
        let cmd_c: Vec<&[u8]> = cmd.chunks(63).collect();
        let chunks_n = cmd_c.len();
        for (i, chunk) in cmd_c.into_iter().enumerate() {
            let mut buf = Vec::new();
            buf.push((if i==0 {0x80} else {0x00})
                + (if i==chunks_n-1 {0x40} else {0x00}) + chunk.len() as u8);
            buf.extend(chunk);
            buf.extend(iter::repeat(0x00).take(63-chunk.len()));
            self.device.write_output_report(&buf).await?;
        }
        Ok(())
    }

    pub async fn receive(&mut self) -> HidResult<Vec<u8>> {
        let mut recvd = Vec::new();
        let mut buf = Vec::new();
        buf.resize(64, 0x00);
        while buf[0] & 0x40 == 0 {
            loop {
                self.device.read_input_report(&mut buf).await?;
                if recvd.len() != 0 || buf[0] & 0x80 != 0 {
                    break
                }
            }
            recvd.extend(&buf[1..((buf[0] & 0x3f)+1) as usize]);
        }
        Ok(recvd)
    }


    pub async fn reset(&mut self) -> HidResult<()> {
        self.cmd(b"\x1ba").await?;
        Ok(())
    }

//    pub async fn read_iso(&mut self) -> HidResult<()> {
//        self.cmd(b"\x1br").await?;
//        Ok(())
//    }

    pub async fn end(&mut self) -> HidResult<()> {
        self.cmd(b"\x1b\xa4").await?;
        Ok(())
    }
}

