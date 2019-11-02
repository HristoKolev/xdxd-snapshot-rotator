use serde_json::json;
use handlebars::Handlebars;

use super::email;
use super::prelude::*;
use crate::global::logger;
use crate::global::app_config::VmConfig;

pub fn send_error_report(error: &CustomError) -> Result {

    let app_config = app_config();

    let subject = format!(
        "[FAILURE] xdxd-snapshot-rotator | An error occurred on `{}`.",
        app_config.hostname
    );

    let html_template = include_str!("email-template.html");

    let logs = logger().get_logs()?.join("\n");

    let registry = Handlebars::new();

    let now = app_start_time();

    let report_content = registry.render_template(
        html_template,
        &json!({
            "app_config": app_config,
            "timestamp": now.format("%+").to_string(),
            "formatted_error": format!("{:#?}", error),
            "logs": logs,
         })
    )?;

    send_mail(&subject, &report_content)?;

    Ok(())
}


pub fn send_success_report(vm: &VmConfig) -> Result {

    let app_config = app_config();

    let subject = format!(
        "[SUCCESS] xdxd-snapshot-rotator | Snapshot was created for vm `{}` on host `{}`.",
        vm.vm_name,
        app_config.hostname
    );

    let html_template = include_str!("email-template.html");

    let logs = logger().get_logs()?.join("\n");

    let registry = Handlebars::new();

    let now = app_start_time();

    let report_content = registry.render_template(
        html_template,
        &json!({
            "app_config": app_config,
            "timestamp": now.format("%+").to_string(),
            "logs": logs,
         })
    )?;

    send_mail(&subject, &report_content)?;

    Ok(())
}

fn send_mail(subject: &str, content: &str) -> Result {

    let app_config = app_config();

    let email_client = email::EmailClient::new(
        &app_config.email_config.smtp_username,
        &app_config.email_config.smtp_password,
        &app_config.email_config.smtp_host,
        app_config.email_config.smtp_port,
    );

    let message = email::EmailMessage::new(
        app_config.email_config.notification_emails.clone(),
        subject,
        content,
    );

    email_client.send(&message)?;

    Ok(())
}
