use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::prelude::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::widgets::block::Position;
use crate::gamelogic::game_screens::GameScreen;
use crate::gamelogic::konst;
use crate::gameobjects::item_handler::{Item, ItemsTypes};
use crate::gameobjects::player::Player;



pub(crate) fn create_log(log: Vec<String>) -> List<'static>{
    if(log.len() <= konst::LOGLENGTH) {
       List::new(log)
    } else {
       List::new( log[(log.len() - konst::LOGLENGTH)..log.len()]
                      .iter()
                      .cloned())
    }





} 


/*
pub(crate) fn draw_inventory(frame: &mut Frame, game_screen: &GameScreen) {
    match &game_screen.content{
        Some(content_layout) => {
            let player = Player::player_ref().lock().unwrap();

            let inventory = player.get_inventory();


            let items_layout = Layout::default()
                .constraints(
                    (0..(inventory.len() + 1))
                        .map(|_| Constraint::Percentage((100 / (inventory.len() + 1)) as u16))
                        .collect::<Vec<Constraint>>(),
                )
                .direction(Vertical)
                .split(content_layout.split(frame.area())[0]);

            for i in 0..(inventory.len() + 1) {
                let row_layout = Layout::default()
                    .direction(Horizontal)
                    .constraints([Constraint::Ratio(1, 4); 4])
                    .split(items_layout[i]);

                if (i == 0) {
                    frame.render_widget(Paragraph::new("Index"), row_layout[0]);
                    frame.render_widget(Paragraph::new("Name"), row_layout[1]);
                    frame.render_widget(Paragraph::new("Rarity"), row_layout[2]);
                    frame.render_widget(Paragraph::new("Notes"), row_layout[2]);
                } else {
                    let mut name = String::new();
                    let mut note = String::new();
                    let mut rarity = String::new();

                    match inventory.get(i - 1).unwrap() {
                        ItemsTypes::EquipItem(item) => {
                            name = item.get_name().parse().unwrap();
                            rarity = item.get_rarity().to_string().parse().unwrap();
                            note = format!("+{} AD", item.get_armor_buff())
                        }
                        ItemsTypes::WeaponItem(item) => {
                            name = item.get_name().parse().unwrap();
                            rarity = item.get_rarity().to_string().parse().unwrap();
                            note = format!("+{} DMG", item.get_bonus_dmg())
                        }
                        ItemsTypes::ConsumableItem(item) => {
                            name = item.get_name().parse().unwrap();
                            rarity = item.get_rarity().to_string().parse().unwrap();
                            if (item.get_name().to_ascii_lowercase().contains("heal")) {
                                note = format!("+{} Healing", item.get_buf())
                            }
                        }
                        ItemsTypes::TreasureItem(item) => {
                            name = item.get_name().parse().unwrap();
                            rarity = item.get_rarity().to_string().parse().unwrap();
                            note = format!("+{} AD", item.get_passiv().to_string())
                        }

                        ItemsTypes::InventorySlot(item) => {
                            name = item.get_name().parse().unwrap();
                            rarity = item.get_rarity().to_string().parse().unwrap();
                            note = item.get_des().parse().unwrap();
                        }
                    }

                    frame.render_widget(Paragraph::new(format!("{}", i - 1)), row_layout[0]);
                    frame.render_widget(Paragraph::new(name), row_layout[1]);
                    frame.render_widget(Paragraph::new(rarity), row_layout[2]);
                    frame.render_widget(Paragraph::new(note), row_layout[2]);
                }
            }

            let help = Paragraph::new(Text::from(
                konst::INVENTORYHELP
                    .split("\n")
                    .map(|txt| Line::from(Span::from(txt)))
                    .collect::<Vec<Line>>(),
            ))
                .block(Block::new().title("Inventory Help").borders(Borders::ALL));

            let equipment_display = Block::new().borders(Borders::ALL);

            let mut equipment_layout = Layout::default()
                .constraints([
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ])
                .direction(Vertical)
                .split(equipment_display.inner(content_layout[1]));

            if (player.get_inspect().0) {
                let item = inventory.get(player.get_inspect().1 as usize).unwrap();
                let itemdes = item
                    .get_des()
                    .split("\\")
                    .map(|line| Line::from(Span::from(line)))
                    .collect::<Vec<Line>>();

                let item_list = List::new(vec![
                    ListItem::from(Line::from("")),
                    ListItem::from(Line::from(Span::from(format!("Name: {}", item.get_name())))),
                    ListItem::from(Line::from("")),
                    ListItem::from(Line::from("Des:")),
                    ListItem::from(itemdes),
                    ListItem::from(Line::from("")),
                    ListItem::from(Line::from(format!(
                        "Rarity: {}",
                        item.get_rarity().to_string()
                    ))),
                ]);

                frame.render_widget(item_list, equipment_display.inner(content_layout[1]));
                frame.render_widget(
                    equipment_display.title(format!(
                        "Item: {}",
                        inventory
                            .get(player.get_inspect().1 as usize)
                            .unwrap()
                            .get_name()
                    )),
                    content_layout[1],
                );
            } else {
                let equipment_list = List::new(vec![
                    ListItem::from(Line::from("")),
                    ListItem::from(Line::from(Span::from(format!(
                        "Head: {}",
                        player.get_equipment_from_slot("head".into()).get_name()
                    )))),
                    ListItem::from(Line::from("")),
                    ListItem::from(Line::from(Span::from(format!(
                        "Torso: {}",
                        player.get_equipment_from_slot("Torso".into()).get_name()
                    )))),
                    ListItem::from(Line::from("")),
                    ListItem::from(Line::from(Span::from(format!(
                        "Hands: {}",
                        player.get_equipment_from_slot("Hands".into()).get_name()
                    )))),
                    ListItem::from(Line::from("")),
                    ListItem::from(Line::from(Span::from(format!(
                        "Pants: {}",
                        player.get_equipment_from_slot("Pants".into()).get_name()
                    )))),
                    ListItem::from(Line::from("")),
                    ListItem::from(Line::from(Span::from(format!(
                        "Shoes: {}",
                        player.get_equipment_from_slot("Shoes".into()).get_name()
                    )))),
                ]);

                frame.render_widget(equipment_list, equipment_display.inner(content_layout[1]));

                frame.render_widget(
                    equipment_display
                        .title("Equipment")
                        .title_position(Position::Top),
                    content_layout[1],
                );
            }

            frame.render_widget(help, content_layout[1]);
        }
        _ => {}
    }

}
 */
