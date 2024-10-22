#[cfg(test)]
mod tests {
    use lettre::message::header::ContentType;
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};

    #[test]
    fn test_email() {

        // 创建邮件内容
        let email = Message::builder()
            .from("Your Name <office@bonviewpress.com>".parse().unwrap())
            .reply_to("hnbcao@qq.com".parse().unwrap())
            .to("Recipient Name <hnbcao@qq.com>".parse().unwrap())
            .subject("Rust Email")
            .header(ContentType::TEXT_PLAIN)
            .body(String::from("Hello, this is a test email from Rust!"))
            .unwrap();

        // 设置 SMTP 客户端的认证信息 dhykhdydgaiscbed
        let creds = Credentials::new("office@bonviewpress.com".to_owned(), "AfjvEBnAfwmmQ32C".to_owned());

        // 建立到 SMTP 服务器的连接
        let mailer = SmtpTransport::relay("smtp.exmail.qq.com")
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {e:?}"),
        }
    }

    #[test]
    fn test_email2() {

        // 创建邮件内容
        let email = Message::builder()
            .from("Your Name <hnbcao@foxmail.com>".parse().unwrap())
            .reply_to("hnbcao@qq.com".parse().unwrap())
            .to("Recipient Name <hnbcao@qq.com>".parse().unwrap())
            .subject("Rust Email")
            .header(ContentType::TEXT_PLAIN)
            .body(String::from("Hello, this is a test email from Rust!"))
            .unwrap();

        // 设置 SMTP 客户端的认证信息 dhykhdydgaiscbed
        let creds = Credentials::new("hnbcao@foxmail.com".to_owned(), "dhykhdydgaiscbed".to_owned());

        // 建立到 SMTP 服务器的连接
        let mailer = SmtpTransport::relay("smtp.qq.com")
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {e:?}"),
        }
    }
}