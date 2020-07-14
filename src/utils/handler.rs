use protonsdk_variant::*;
use std::collections::HashMap;

pub fn on_login(peer: &mut enet::Peer<()>, _: Option<HashMap<String, String>>) {
    let packet: crate::GamePacket = crate::GamePacket::new(0, -1);

    packet.combine(var_fn!(
        "OnSuperMainStartAcceptLogonHrdxs47254722215a",
        0x55555555,
        "ubistatic-a.akamaihd.net",
        "0098/CDNContent74/cache",
        "cc.cz.madkite.freedom org.aqua.gg idv.aqua.bulldog com.cih.gamecih2 com.cih.gamecih com.cih.game_cih cn.maocai.gamekiller com.gmd.speedtime org.dax.attack com.x0.strai.frep com.x0.strai.free org.cheatengine.cegui org.sbtools.gamehack com.skgames.traffikrider org.sbtoods.gamehaca com.skype.ralder org.cheatengine.cegui.xx.multi1458919170111 com.prohiro.macro me.autotouch.autotouch com.cygery.repetitouch.free com.cygery.repetitouch.pro com.proziro.zacro com.slash.gamebuster",
        "proto=84|choosemusic=audio/mp3/about_theme.mp3|active_holiday=0|server_tick=226933875|clash_active=0|drop_lavacheck_faster=1|isPayingUser=0|"
    )).send(0, peer, None);
}