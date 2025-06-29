use tokio::io::AsyncWriteExt;

const DISABLE_CORES: &str = "/sys/devices/system/cpu/cpu{cpu_n}/online";

pub async fn disable_cores(disable: usize, of: usize) -> anyhow::Result<()> {
    if disable >= of {
        return Err(anyhow::anyhow!(
            "Disable cores must be less than total cores"
        ));
    }
    
    let futures = (1..=disable).map(|i| {
        let cpu_num = of - i;
        change_core_state(cpu_num, false)
    });
    
    futures::future::try_join_all(futures).await?;
    
    Ok(())
}

pub async fn enable_all_cores(of: usize) -> anyhow::Result<()> {
    let futures = (1..of).map(|i| change_core_state(i, true));
    
    futures::future::try_join_all(futures).await?;
    
    Ok(())
}


pub async fn get_core_states(of: usize) -> anyhow::Result<Vec<bool>> {
    let futures = (1..of).map(|i| get_core_state(i));
    
    let mut states = futures::future::try_join_all(futures).await?;
    
    states.insert(0, true); // CPU0 is always online
    
    Ok(states)
}

pub async fn change_core_state(cpu_n: usize, state: bool) -> anyhow::Result<()> {
    let path = get_core_path(cpu_n);

    let value = if state { "1" } else { "0" };
    
    let mut file = tokio::fs::OpenOptions::new()
        .write(true)
        .open(&path)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to open {}: {}", path, e))?;
    
    file.write_all(value.as_bytes())
        .await
        .map_err(|e| anyhow::anyhow!("Failed to write to {}: {}", path, e))?;

    Ok(())
}

pub fn get_core_path(cpu_n: usize) -> String {
    DISABLE_CORES.replace("{cpu_n}", &cpu_n.to_string())
}

pub async fn get_core_state(cpu_n: usize) -> anyhow::Result<bool> {
    let path = get_core_path(cpu_n);
    
    let content = tokio::fs::read_to_string(&path)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to read {}: {}", path, e))?;
    
    let state = content.trim() == "1";
    
    Ok(state)
        
}
