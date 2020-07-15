use protonsdk_variant::*;
use std::collections::HashMap;

#[path = "./world.rs"]
mod world;

use world::World;

#[derive(Debug)]
pub struct Handler {
    hash: Option<u32>,
    items_dat: Option<Vec<u8>>
}

impl Handler {
    pub fn on_login(self, peer: &mut enet::Peer<()>, _: Option<HashMap<String, String>>) {
        let packet: crate::GamePacket = crate::GamePacket::new(0, -1);
        let hash: u32 = match self.hash {
            Some(h) => h,
            None => 0x55555555
        };

        packet.combine(var_fn!(
            "OnSuperMainStartAcceptLogonHrdxs47254722215a",
            hash,
            "ubistatic-a.akamaihd.net",
            "0098/CDNContent74/cache/",
            "cc.cz.madkite.freedom org.aqua.gg idv.aqua.bulldog com.cih.gamecih2 com.cih.gamecih com.cih.game_cih cn.maocai.gamekiller com.gmd.speedtime org.dax.attack com.x0.strai.frep com.x0.strai.free org.cheatengine.cegui org.sbtools.gamehack com.skgames.traffikrider org.sbtoods.gamehaca com.skype.ralder org.cheatengine.cegui.xx.multi1458919170111 com.prohiro.macro me.autotouch.autotouch com.cygery.repetitouch.free com.cygery.repetitouch.pro com.proziro.zacro com.slash.gamebuster",
            "proto=84|choosemusic=audio/mp3/about_theme.mp3|active_holiday=0|server_tick=226933875|clash_active=0|drop_lavacheck_faster=1|isPayingUser=0|"
        )).send(0, peer, None);
    }

    pub fn on_request_items_dat(self, peer: &mut enet::Peer<()>, _: Option<HashMap<String, String>>) {
        let packet: crate::GamePacket = crate::GamePacket::from(self.items_dat.unwrap());
        packet.send(0, peer, None);
    }

    pub fn on_enter_game(self, peer: &mut enet::Peer<()>, _: Option<HashMap<String, String>>) {
        let packet: crate::GamePacket = crate::GamePacket::new(0, -1);
        
        packet.combine(var_fn!(
            "OnRequestWorldSelectMenu",
            "default|\nadd_button|Showing: `wWorlds``|_catselect_|0.6|3529161471|\n"
        )).send(0, peer, None);
    }

    pub fn on_join_request(self, peer: &mut enet::Peer<()>, received: Option<HashMap<String, String>>) {
        let data: HashMap<String, String> = received.unwrap(); // just overwrite it
        
        let world: World = World::new(data.get("name").unwrap().as_str(), 100, 60);
        let world_packet: Vec<u8> = world.packet(); // create the packet
        let mut packet: crate::GamePacket = crate::GamePacket::from(world_packet);

        packet.send(0, peer, None); // send world packet

        packet = crate::GamePacket::new(0, -1); // recreate packet
        packet.combine(var_fn!(
            "OnSpawn",
            "spawn|avatar\nnetID|1\nuserID|1\ncolrect|0|0|20|30\nposXY|3040|700\nname|`#Test``\ncountry|ph\ninvis|0\nmstate|0\nsmstate|0\ntype|local\n"
        )).send(0, peer, None); // send OnSpawn packet
    }

    pub fn new(hash: Option<u32>, items_dat: Option<Vec<u8>>) -> Handler {
        Handler {
            hash,
            items_dat
        }
    }
}