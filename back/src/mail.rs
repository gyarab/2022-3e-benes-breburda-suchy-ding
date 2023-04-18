use lettre::{AsyncSmtpTransport, AsyncStd1Executor, message::Mailbox, transport::smtp::authentication::Credentials, Message, AsyncTransport};
use anyhow::anyhow;

#[derive(Clone)]
pub struct Mailer {
    pub smtp: AsyncSmtpTransport<AsyncStd1Executor>,
    pub from_address: Mailbox
}

impl Mailer {
    pub fn new(host: &str, username: String, password: String, from: Option<&str>) -> Self {
        let smtp: AsyncSmtpTransport<AsyncStd1Executor> = AsyncSmtpTransport::<AsyncStd1Executor>::relay(host)
            .unwrap()
            .credentials(Credentials::new(username.clone(), password.clone()))
            .build();
        Mailer {
            smtp,
            from_address: from.unwrap_or(&username).parse().unwrap()
        }
    }

    pub async fn send_mail_raw(&self, msg: Message) -> anyhow::Result<()> {
        match self.smtp.send(msg).await {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("failed to send email {}", e))
        }
    }

    pub async fn send_mail(&self, to: &str, subject: &str, body: String) -> anyhow::Result<()>{
        let msg = Message::builder()
            .from(self.from_address.clone())
            .to(to.parse().unwrap())
            .subject(subject)
            .body(body)
            .unwrap();
        self.send_mail_raw(msg).await
    }
}
