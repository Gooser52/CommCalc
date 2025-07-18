slint::include_modules!();

fn calculate_commission_amount(gbp_billings: f64) -> f64 {
    let mut commission = 0.0;
    let mut remaining = gbp_billings;
    
    // 0% on first £5,000
    if remaining > 5000.0 {
        remaining -= 5000.0;
        
        // 10% on next £5,000 (£5,001 - £10,000)
        if remaining > 5000.0 {
            commission += 5000.0 * 0.10;
            remaining -= 5000.0;
            
            // 20% on next £5,000 (£10,001 - £15,000)
            if remaining > 5000.0 {
                commission += 5000.0 * 0.20;
                remaining -= 5000.0;
                
                // 25% on next £5,000 (£15,001 - £20,000)
                if remaining > 5000.0 {
                    commission += 5000.0 * 0.25;
                    remaining -= 5000.0;
                    
                    // 30% on next £10,000 (£20,001 - £30,000)
                    if remaining > 10000.0 {
                        commission += 10000.0 * 0.30;
                        remaining -= 10000.0;
                        
                        // 35% on next £10,000 (£30,001 - £40,000)
                        if remaining > 10000.0 {
                            commission += 10000.0 * 0.35;
                            remaining -= 10000.0;
                            
                            // 40% on everything above £40,000
                            if remaining > 0.0 {
                                commission += remaining * 0.40;
                            }
                        } else {
                            commission += remaining * 0.35;
                        }
                    } else {
                        commission += remaining * 0.30;
                    }
                } else {
                    commission += remaining * 0.25;
                }
            } else {
                commission += remaining * 0.20;
            }
        } else {
            commission += remaining * 0.10;
        }
    }
    
    commission
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    
    ui.on_show_billings(move |string| {
        let ui = ui_handle.unwrap();
        let gbp_billings: f64 = string.trim().parse().unwrap_or(0.0);
        
        let commission_amount = calculate_commission_amount(gbp_billings);
        let effective_rate = if gbp_billings > 0.0 {
            (commission_amount / gbp_billings) * 100.0
        } else {
            0.0
        };
        
        let result = format!(
            "GBP Billings: £{:.2}\nCommission Amount: £{:.2}\nEffective Rate: {:.2}%", 
            gbp_billings, 
            commission_amount,
            effective_rate
        );
        
        ui.set_display_text(result.into());
    });
    
    ui.run()
}