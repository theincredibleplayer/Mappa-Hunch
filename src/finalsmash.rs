use super::*;




unsafe extern "C" fn game_finalstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::CHECK_VALID_FINAL_START_CAMERA(agent, 0, 7, 33, 0, 0, 0);
        macros::SLOW_OPPONENT(agent, 400.0, 160.0);
        macros::FT_SET_FINAL_FEAR_FACE(agent, 140);
        macros::FILL_SCREEN_MODEL_COLOR(agent, 0, 20, 0.1, 0.1, 0.1, 0, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, *EFFECT_SCREEN_PRIO_FINAL);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FT_START_CUTIN(agent);
        if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0  {
            if macros::is_excute(agent) {
                MotionModule::set_flip(agent.module_accessor,true,true,false);
                macros::REQ_FINAL_START_CAMERA_arg3(agent,Hash40::new("d04finalstart.nuanmb"), false, false);
            }
        }else{
            if macros::is_excute(agent) {
                macros::REQ_FINAL_START_CAMERA_arg3(agent,Hash40::new("d04finalstart.nuanmb"), false, false);
            }
        }
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        macros::CAM_ZOOM_OUT(agent);
    }
    frame(agent.lua_state_agent, 146.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_GANOND, false, -1);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_aura_dark"), true, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_shield_smoke"), true, true);


    }

}

unsafe extern "C" fn expression_finalstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        START_INFO_FLASH_EYE(agent.lua_state_agent);
    }
}

unsafe extern "C" fn effect_finalstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_final_aura"), false, true);
        macros::EFFECT(agent, Hash40::new("ganon_final_spark"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.2, 0.0, 0.0);
        macros::EFFECT(agent, Hash40::new("ganon_final_spark"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("ganon_final_spark"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 0.1);
    }
    frame(agent.lua_state_agent,2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_purple_s"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.8, false);
        macros::EFFECT(agent, Hash40::new("demon_heavens_impact"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);

        macros::LAST_EFFECT_SET_RATE(agent, 0.75);

        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);

        macros::LAST_EFFECT_SET_RATE(agent, 0.75);

    }
    frame(agent.lua_state_agent,51.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_purple"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.8, false);

        macros::EFFECT(agent, Hash40::new("demon_heavens_impact"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);

        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 1.7, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 1.7, 0, 0, 0, 0, 0, 0, false);

        macros::LAST_EFFECT_SET_RATE(agent, 0.75);
    }
    frame(agent.lua_state_agent,98.0);
    if macros::is_excute(agent) {

        macros::EFFECT(agent, Hash40::new("demon_heavens_impact"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);

        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -10, 0, 0, 0, 0, 0, 2.3, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), -10, 0, 0, 0, 0, 0, 2.3, 0, 0, 0, 0, 0, 0, false);

        macros::LAST_EFFECT_SET_RATE(agent, 0.75);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_aura_dark"), Hash40::new("legl"),      0, 0, 0, 0, 0, 0, 2, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_aura_dark"), Hash40::new("legr"),      0, 0, 0, 0, 0, 0, 2, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_aura_dark"), Hash40::new("kneel"),     0, 0, 0, 0, 0, 0, 2, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_aura_dark"), Hash40::new("kneer"),     0, 0, 0, 0, 0, 0, 2, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_aura_dark"), Hash40::new("footl"),     0, 0, 0, 0, 0, 0, 2, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_aura_dark"), Hash40::new("footr"),     0, 0, 0, 0, 0, 0, 2, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_aura_dark"), Hash40::new("waist"),     0, 0, 0, 0, 0, 0, 2, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_aura_dark"), Hash40::new("bust"),      0, 0, 0, 0, 0, 0, 2, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_aura_dark"), Hash40::new("hip"),       0, 0, 0, 0, 0, 0, 2, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_aura_dark"), Hash40::new("shoulderl"), 0, 0, 0, 0, 0, 0, 2, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_aura_dark"), Hash40::new("shoulderr"), 0, 0, 0, 0, 0, 0, 2, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_aura_dark"), Hash40::new("arml"),      0, 0, 0, 0, 0, 0, 2, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_aura_dark"), Hash40::new("armr"),      0, 0, 0, 0, 0, 0, 2, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_aura_dark"), Hash40::new("handl"),     0, 0, 0, 0, 0, 0, 2, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_aura_dark"), Hash40::new("handr"),     0, 0, 0, 0, 0, 0, 2, false);
        
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_dead_dark"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.2, false);

    }
    frame(agent.lua_state_agent, 134.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("ganon_final_transform"), Hash40::new("hip"), 0, 0, 1, 50, 0, 0,0.8, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_finalstart(agent: &mut L2CAgentBase) {
    let random_mappa = smash::app::sv_math::rand(hash40("fighter"), 3);
    if random_mappa == 0 {
        frame(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("mappa_hunch_1"));
            macros::PLAY_SE(agent, Hash40::new("mappa_hunch_sfx"));
        }
    }else if random_mappa == 1{
        frame(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("mappa_hunch_2"));
            macros::PLAY_SE(agent, Hash40::new("mappa_hunch_sfx"));
        }
    }else if random_mappa == 2 {
        frame(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("mappa_hunch_3"));
            macros::PLAY_SE(agent, Hash40::new("mappa_hunch_sfx"));
        }
    }
}



unsafe extern "C" fn game_finalend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::CANCEL_FILL_SCREEN(agent, 0, 60);
        if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0  {
            if macros::is_excute(agent) {
                MotionModule::set_flip(agent.module_accessor,false,true,false);
            }
        }else{
            if macros::is_excute(agent) {

            }
        }
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 1.0);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 0.8);
    }
    frame(agent.lua_state_agent, 75.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 0.9);
    }
}

unsafe extern "C" fn effect_finalend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("ganon_final_transform_end"), Hash40::new("top"), 0, 11, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    for _ in 0..14 {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("ganon_final_transform_light"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 0.8, 15, 15, 15, 0, 0, 0, true);
            macros::BURN_COLOR(agent, 0.1, 1, 2, 0.7);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_FRAME(agent, 5, 0.1, 1, 2, 0);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ganon_final_spark"), true, true);
    }
}



unsafe extern "C" fn effect_attack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *WEAPON_GANON_BEAST_INSTANCE_WORK_ID_FLAG_DASH);
    }

    for _ in 0..3 {
    if macros::is_excute(agent) {
        macros::BURN_COLOR(agent, 1.5, 0.5, 2, 0.5);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::BURN_COLOR_FRAME(agent, 3, 1.5, 0.5, 2, 0);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::BURN_COLOR_NORMAL(agent);
    }
    wait(agent.lua_state_agent, 1.0);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("ganon_final_attack"), -1);
    }
}

unsafe extern "C" fn game_attack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_23_rush_sp"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 45.0, 42, 25, 0, 122, 10.0, 0.0, 12.0, 25.0, None, None, None, 200.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 35.0, 42, 25, 0, 122, 15.0, 0.0, 12.0, 5.0, Some(0.0), Some(12.0), Some(-20.0), 200.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 35.0, 42, 25, 0, 122, 18.0, 0.0, 30.0, 13.0, Some(0.0), Some(30.0), Some(-18.0), 200.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 1, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 2, true, false);
        AttackModule::set_final_finish_cut_in(agent.module_accessor, 0, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(agent.module_accessor, 1, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(agent.module_accessor, 2, true, true, -1.0, false);
    }
}

unsafe extern "C" fn effect_start(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
    macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_final_attack"), Hash40::new("top"), 0, 25, 55, 0, 0, 0, 1.3, false);
    macros::EFFECT_FOLLOW(agent, Hash40::new("sys_speedbooster"), Hash40::new("top"), 0, 36, 55, 0, 0, 0, 1.3, false);
    }
}

pub unsafe extern "C" fn ganon_d(fighter: &mut L2CFighterCommon) {
    unsafe {
        if StatusModule::status_kind(fighter.module_accessor) == *WEAPON_GANON_BEAST_STATUS_KIND_ATTACK {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                let hit_stop = StopModule::get_hit_stop_real_frame(fighter.module_accessor) as i32;
                StopModule::set_hit_stop_frame(fighter.module_accessor, 30, true);
            }
        }
    }
}





pub fn install() {
    unsafe {
        let mut costume = &mut Vec::new();
        unsafe {
            for i in 0..crate::MAPPA_COLORS.len() {
                if crate::MAPPA_COLORS[i] {
                    costume.push(i);
                }
            }
        }
        Agent::new("ganon").set_costume(costume.to_vec())
        .game_acmd("game_finalstart", game_finalstart,Priority::Default)
        .game_acmd("game_finalairstart", game_finalstart,Priority::Default)

        .game_acmd("game_finalend", game_finalend,Priority::Default)
        .game_acmd("game_finalairend", game_finalend,Priority::Default)

        .sound_acmd("sound_finalstart", sound_finalstart,Priority::Default)
        .sound_acmd("sound_finalairstart", sound_finalstart,Priority::Default)

        .effect_acmd("effect_finalstart", effect_finalstart,Priority::Default)
        .effect_acmd("effect_finalairstart", effect_finalstart,Priority::Default)
        
        .expression_acmd("expression_finalstart", expression_finalstart,Priority::Default)
        .expression_acmd("expression_finalairstart", expression_finalstart,Priority::Default)


        .install();


        Agent::new("ganon_ganond").set_costume(costume.to_vec())
        .on_line(Main, ganon_d)
        .game_acmd("game_attack", game_attack,Priority::Default)
        .game_acmd("game_attackair", game_attack,Priority::Default)

        .effect_acmd("effect_attack", effect_attack,Priority::Default)
        .effect_acmd("effect_attackair", effect_attack,Priority::Default)
        
        .effect_acmd("effect_start", effect_start,Priority::Default)
        .effect_acmd("effect_startair", effect_start,Priority::Default)


        .install();
    }
}
