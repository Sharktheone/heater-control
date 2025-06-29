use zbus::{Connection, MatchRule, MessageStream};
use zbus::export::ordered_stream::OrderedStreamExt;
use zbus::fdo::DBusProxy;
use zbus::message::Type;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let connection = Connection::session().await?;

    let proxy = DBusProxy::new(&connection).await?;
    
    let rule = MatchRule::builder()
        .msg_type(Type::Signal)
        .interface("org.freedesktop.ScreenSaver")?
        .build();

    
    let mut stream = MessageStream::for_match_rule(rule, &connection, None)
        .await?;
    
    while let Some(msg) = stream.next().await {
        
        if let Ok(signal) = msg{
            dbg!(signal);
        } else {
            eprintln!("Failed to parse message as ScreenSaver signal");
        }
        
        
    }


    Ok(())
}
