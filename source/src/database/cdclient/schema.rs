table! {
    AICombatRoles (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        preferredRole -> Nullable<Integer>,
        specifiedMinRangeNOUSE -> Nullable<Float>,
        specifiedMaxRangeNOUSE -> Nullable<Float>,
        specificMinRange -> Nullable<Float>,
        specificMaxRange -> Nullable<Float>,
    }
}

table! {
    AccessoryDefaultLoc (efId) {
        efId -> Integer,
        GroupID -> Nullable<Integer>,
        Description -> Nullable<Text>,
        Pos_X -> Nullable<Float>,
        Pos_Y -> Nullable<Float>,
        Pos_Z -> Nullable<Float>,
        Rot_X -> Nullable<Float>,
        Rot_Y -> Nullable<Float>,
        Rot_Z -> Nullable<Float>,
    }
}

table! {
    Activities (efId) {
        efId -> Integer,
        ActivityID -> Nullable<Integer>,
        locStatus -> Nullable<Integer>,
        instanceMapID -> Nullable<Integer>,
        minTeams -> Nullable<Integer>,
        maxTeams -> Nullable<Integer>,
        minTeamSize -> Nullable<Integer>,
        maxTeamSize -> Nullable<Integer>,
        waitTime -> Nullable<Integer>,
        startDelay -> Nullable<Integer>,
        requiresUniqueData -> Nullable<Integer>,
        leaderboardType -> Nullable<Integer>,
        localize -> Nullable<Integer>,
        optionalCostLOT -> Nullable<Integer>,
        optionalCostCount -> Nullable<Integer>,
        showUIRewards -> Nullable<Integer>,
        CommunityActivityFlagID -> Nullable<Integer>,
        gate_version -> Nullable<Text>,
        noTeamLootOnDeath -> Nullable<Integer>,
        optionalPercentage -> Nullable<Float>,
    }
}

table! {
    ActivityRewards (efId) {
        efId -> Integer,
        objectTemplate -> Nullable<Integer>,
        ActivityRewardIndex -> Nullable<Integer>,
        activityRating -> Nullable<Integer>,
        LootMatrixIndex -> Nullable<Integer>,
        CurrencyIndex -> Nullable<Integer>,
        ChallengeRating -> Nullable<Integer>,
        description -> Nullable<Text>,
    }
}

table! {
    ActivityText (efId) {
        efId -> Integer,
        activityID -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        localize -> Nullable<Integer>,
        locStatus -> Nullable<Integer>,
        gate_version -> Nullable<Text>,
    }
}

table! {
    AnimationIndex (efId) {
        efId -> Integer,
        animationGroupID -> Nullable<Integer>,
        description -> Nullable<Text>,
        groupType -> Nullable<Text>,
    }
}

table! {
    Animations (efId) {
        efId -> Integer,
        animationGroupID -> Nullable<Integer>,
        animation_type -> Nullable<Text>,
        animation_name -> Nullable<Text>,
        chance_to_play -> Nullable<Float>,
        min_loops -> Nullable<Integer>,
        max_loops -> Nullable<Integer>,
        animation_length -> Nullable<Float>,
        hideEquip -> Nullable<Integer>,
        ignoreUpperBody -> Nullable<Integer>,
        restartable -> Nullable<Integer>,
        face_animation_name -> Nullable<Text>,
        priority -> Nullable<Float>,
        blendTime -> Nullable<Float>,
    }
}

table! {
    BaseCombatAIComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        behaviorType -> Nullable<Integer>,
        combatRoundLength -> Nullable<Float>,
        combatRole -> Nullable<Integer>,
        minRoundLength -> Nullable<Float>,
        maxRoundLength -> Nullable<Float>,
        tetherSpeed -> Nullable<Float>,
        pursuitSpeed -> Nullable<Float>,
        combatStartDelay -> Nullable<Float>,
        softTetherRadius -> Nullable<Float>,
        hardTetherRadius -> Nullable<Float>,
        spawnTimer -> Nullable<Float>,
        tetherEffectID -> Nullable<Integer>,
        ignoreMediator -> Nullable<Integer>,
        aggroRadius -> Nullable<Float>,
        ignoreStatReset -> Nullable<Integer>,
        ignoreParent -> Nullable<Integer>,
    }
}

table! {
    BehaviorEffect (efId) {
        efId -> Integer,
        effectID -> Nullable<Integer>,
        effectType -> Nullable<Text>,
        effectName -> Nullable<Text>,
        trailID -> Nullable<Integer>,
        pcreateDuration -> Nullable<Float>,
        animationName -> Nullable<Text>,
        attachToObject -> Nullable<Integer>,
        boneName -> Nullable<Text>,
        useSecondary -> Nullable<Integer>,
        cameraEffectType -> Nullable<Integer>,
        cameraDuration -> Nullable<Float>,
        cameraFrequency -> Nullable<Float>,
        cameraXAmp -> Nullable<Float>,
        cameraYAmp -> Nullable<Float>,
        cameraZAmp -> Nullable<Float>,
        cameraRotFrequency -> Nullable<Float>,
        cameraRoll -> Nullable<Float>,
        cameraPitch -> Nullable<Float>,
        cameraYaw -> Nullable<Float>,
        AudioEventGUID -> Nullable<Text>,
        renderEffectType -> Nullable<Integer>,
        renderEffectTime -> Nullable<Float>,
        renderStartVal -> Nullable<Float>,
        renderEndVal -> Nullable<Float>,
        renderDelayVal -> Nullable<Float>,
        renderValue1 -> Nullable<Float>,
        renderValue2 -> Nullable<Float>,
        renderValue3 -> Nullable<Float>,
        renderRGBA -> Nullable<Text>,
        renderShaderVal -> Nullable<Integer>,
        motionID -> Nullable<Integer>,
        meshID -> Nullable<Integer>,
        meshDuration -> Nullable<Float>,
        meshLockedNode -> Nullable<Text>,
    }
}

table! {
    BehaviorParameter (efId) {
        efId -> Integer,
        behaviorID -> Nullable<Integer>,
        parameterID -> Nullable<Text>,
        value -> Nullable<Float>,
    }
}

table! {
    BehaviorTemplate (efId) {
        efId -> Integer,
        behaviorID -> Nullable<Integer>,
        templateID -> Nullable<Integer>,
        effectID -> Nullable<Integer>,
        effectHandle -> Nullable<Text>,
    }
}

table! {
    BehaviorTemplateName (efId) {
        efId -> Integer,
        templateID -> Nullable<Integer>,
        name -> Nullable<Text>,
    }
}

table! {
    Blueprints (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        description -> Nullable<Text>,
        accountid -> Nullable<Integer>,
        characterid -> Nullable<Integer>,
        price -> Nullable<Integer>,
        rating -> Nullable<Integer>,
        categoryid -> Nullable<Integer>,
        lxfpath -> Nullable<Text>,
        deleted -> Nullable<Integer>,
        created -> Nullable<Integer>,
        modified -> Nullable<Integer>,
    }
}

table! {
    BrickColors (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        red -> Nullable<Float>,
        green -> Nullable<Float>,
        blue -> Nullable<Float>,
        alpha -> Nullable<Float>,
        legopaletteid -> Nullable<Integer>,
        description -> Nullable<Text>,
        validTypes -> Nullable<Integer>,
        validCharacters -> Nullable<Integer>,
        factoryValid -> Nullable<Integer>,
    }
}

table! {
    BrickIDTable (efId) {
        efId -> Integer,
        NDObjectID -> Nullable<Integer>,
        LEGOBrickID -> Nullable<Integer>,
    }
}

table! {
    BuffDefinitions (efId) {
        efId -> Integer,
        ID -> Nullable<Integer>,
        Priority -> Nullable<Float>,
        UIIcon -> Nullable<Text>,
    }
}

table! {
    BuffParameters (efId) {
        efId -> Integer,
        BuffID -> Nullable<Integer>,
        ParameterName -> Nullable<Text>,
        NumberValue -> Nullable<Float>,
        StringValue -> Nullable<Text>,
        EffectID -> Nullable<Integer>,
    }
}

table! {
    Camera (efId) {
        efId -> Integer,
        camera_name -> Nullable<Text>,
        pitch_angle_tolerance -> Nullable<Float>,
        starting_zoom -> Nullable<Float>,
        zoom_return_modifier -> Nullable<Float>,
        pitch_return_modifier -> Nullable<Float>,
        tether_out_return_modifier -> Nullable<Float>,
        tether_in_return_multiplier -> Nullable<Float>,
        verticle_movement_dampening_modifier -> Nullable<Float>,
        return_from_incline_modifier -> Nullable<Float>,
        horizontal_return_modifier -> Nullable<Float>,
        yaw_behavior_speed_multiplier -> Nullable<Float>,
        camera_collision_padding -> Nullable<Float>,
        glide_speed -> Nullable<Float>,
        fade_player_min_range -> Nullable<Float>,
        min_movement_delta_tolerance -> Nullable<Float>,
        min_glide_distance_tolerance -> Nullable<Float>,
        look_forward_offset -> Nullable<Float>,
        look_up_offset -> Nullable<Float>,
        minimum_vertical_dampening_distance -> Nullable<Float>,
        maximum_vertical_dampening_distance -> Nullable<Float>,
        minimum_ignore_jump_distance -> Nullable<Float>,
        maximum_ignore_jump_distance -> Nullable<Float>,
        maximum_auto_glide_angle -> Nullable<Float>,
        minimum_tether_glide_distance -> Nullable<Float>,
        yaw_sign_correction -> Nullable<Float>,
        set_1_look_forward_offset -> Nullable<Float>,
        set_1_look_up_offset -> Nullable<Float>,
        set_2_look_forward_offset -> Nullable<Float>,
        set_2_look_up_offset -> Nullable<Float>,
        set_0_speed_influence_on_dir -> Nullable<Float>,
        set_1_speed_influence_on_dir -> Nullable<Float>,
        set_2_speed_influence_on_dir -> Nullable<Float>,
        set_0_angular_relaxation -> Nullable<Float>,
        set_1_angular_relaxation -> Nullable<Float>,
        set_2_angular_relaxation -> Nullable<Float>,
        set_0_position_up_offset -> Nullable<Float>,
        set_1_position_up_offset -> Nullable<Float>,
        set_2_position_up_offset -> Nullable<Float>,
        set_0_position_forward_offset -> Nullable<Float>,
        set_1_position_forward_offset -> Nullable<Float>,
        set_2_position_forward_offset -> Nullable<Float>,
        set_0_FOV -> Nullable<Float>,
        set_1_FOV -> Nullable<Float>,
        set_2_FOV -> Nullable<Float>,
        set_0_max_yaw_angle -> Nullable<Float>,
        set_1_max_yaw_angle -> Nullable<Float>,
        set_2_max_yaw_angle -> Nullable<Float>,
        set_1_fade_in_camera_set_change -> Nullable<Integer>,
        set_1_fade_out_camera_set_change -> Nullable<Integer>,
        set_2_fade_in_camera_set_change -> Nullable<Integer>,
        set_2_fade_out_camera_set_change -> Nullable<Integer>,
        input_movement_scalar -> Nullable<Float>,
        input_rotation_scalar -> Nullable<Float>,
        input_zoom_scalar -> Nullable<Float>,
        minimum_pitch_desired -> Nullable<Float>,
        maximum_pitch_desired -> Nullable<Float>,
        minimum_zoom -> Nullable<Float>,
        maximum_zoom -> Nullable<Float>,
        horizontal_rotate_tolerance -> Nullable<Float>,
        horizontal_rotate_modifier -> Nullable<Float>,
    }
}

table! {
    CelebrationParameters (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        animation -> Nullable<Text>,
        backgroundObject -> Nullable<Integer>,
        duration -> Nullable<Float>,
        subText -> Nullable<Text>,
        mainText -> Nullable<Text>,
        iconID -> Nullable<Integer>,
        celeLeadIn -> Nullable<Float>,
        celeLeadOut -> Nullable<Float>,
        cameraPathLOT -> Nullable<Integer>,
        pathNodeName -> Nullable<Text>,
        ambientR -> Nullable<Float>,
        ambientG -> Nullable<Float>,
        ambientB -> Nullable<Float>,
        directionalR -> Nullable<Float>,
        directionalG -> Nullable<Float>,
        directionalB -> Nullable<Float>,
        specularR -> Nullable<Float>,
        specularG -> Nullable<Float>,
        specularB -> Nullable<Float>,
        lightPositionX -> Nullable<Float>,
        lightPositionY -> Nullable<Float>,
        lightPositionZ -> Nullable<Float>,
        blendTime -> Nullable<Float>,
        fogColorR -> Nullable<Float>,
        fogColorG -> Nullable<Float>,
        fogColorB -> Nullable<Float>,
        musicCue -> Nullable<Text>,
        soundGUID -> Nullable<Text>,
        mixerProgram -> Nullable<Text>,
    }
}

table! {
    ChoiceBuildComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        selections -> Nullable<Text>,
        imaginationOverride -> Nullable<Integer>,
    }
}

table! {
    CollectibleComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        requirement_mission -> Nullable<Integer>,
    }
}

table! {
    ComponentsRegistry (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        component_type -> Nullable<Integer>,
        component_id -> Nullable<Integer>,
    }
}

table! {
    ControlSchemes (efId) {
        efId -> Integer,
        control_scheme -> Nullable<Integer>,
        scheme_name -> Nullable<Text>,
        rotation_speed -> Nullable<Float>,
        walk_forward_speed -> Nullable<Float>,
        walk_backward_speed -> Nullable<Float>,
        walk_strafe_speed -> Nullable<Float>,
        walk_strafe_forward_speed -> Nullable<Float>,
        walk_strafe_backward_speed -> Nullable<Float>,
        run_backward_speed -> Nullable<Float>,
        run_strafe_speed -> Nullable<Float>,
        run_strafe_forward_speed -> Nullable<Float>,
        run_strafe_backward_speed -> Nullable<Float>,
        keyboard_zoom_sensitivity -> Nullable<Float>,
        keyboard_pitch_sensitivity -> Nullable<Float>,
        keyboard_yaw_sensitivity -> Nullable<Float>,
        mouse_zoom_wheel_sensitivity -> Nullable<Float>,
        x_mouse_move_sensitivity_modifier -> Nullable<Float>,
        y_mouse_move_sensitivity_modifier -> Nullable<Float>,
        freecam_speed_modifier -> Nullable<Float>,
        freecam_slow_speed_multiplier -> Nullable<Float>,
        freecam_fast_speed_multiplier -> Nullable<Float>,
        freecam_mouse_modifier -> Nullable<Float>,
        gamepad_pitch_rot_sensitivity -> Nullable<Float>,
        gamepad_yaw_rot_sensitivity -> Nullable<Float>,
        gamepad_trigger_sensitivity -> Nullable<Float>,
    }
}

table! {
    CurrencyDenominations (efId) {
        efId -> Integer,
        value -> Nullable<Integer>,
        objectid -> Nullable<Integer>,
    }
}

table! {
    CurrencyTable (efId) {
        efId -> Integer,
        currencyIndex -> Nullable<Integer>,
        npcminlevel -> Nullable<Integer>,
        minvalue -> Nullable<Integer>,
        maxvalue -> Nullable<Integer>,
        id -> Nullable<Integer>,
    }
}

table! {
    DBExclude (efId) {
        efId -> Integer,
        table -> Nullable<Text>,
        column -> Nullable<Text>,
    }
}

table! {
    DeletionRestrictions (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        restricted -> Nullable<Integer>,
        ids -> Nullable<Text>,
        checkType -> Nullable<Integer>,
        localize -> Nullable<Integer>,
        locStatus -> Nullable<Integer>,
        gate_version -> Nullable<Text>,
    }
}

table! {
    DestructibleComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        faction -> Nullable<Integer>,
        factionList -> Nullable<Text>,
        life -> Nullable<Integer>,
        imagination -> Nullable<Integer>,
        LootMatrixIndex -> Nullable<Integer>,
        CurrencyIndex -> Nullable<Integer>,
        level -> Nullable<Integer>,
        armor -> Nullable<Float>,
        death_behavior -> Nullable<Integer>,
        isnpc -> Nullable<Integer>,
        attack_priority -> Nullable<Integer>,
        isSmashable -> Nullable<Integer>,
        difficultyLevel -> Nullable<Integer>,
    }
}

table! {
    DevModelBehaviors (efId) {
        efId -> Integer,
        ModelID -> Nullable<Integer>,
        BehaviorID -> Nullable<Integer>,
    }
}

table! {
    Emotes (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        animationName -> Nullable<Text>,
        iconFilename -> Nullable<Text>,
        channel -> Nullable<Text>,
        command -> Nullable<Text>,
        locked -> Nullable<Integer>,
        localize -> Nullable<Integer>,
        locStatus -> Nullable<Integer>,
        gate_version -> Nullable<Text>,
    }
}

table! {
    EventGating (efId) {
        efId -> Integer,
        eventName -> Nullable<Text>,
        date_start -> Nullable<Integer>,
        date_end -> Nullable<Integer>,
    }
}

table! {
    ExhibitComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        length -> Nullable<Float>,
        width -> Nullable<Float>,
        height -> Nullable<Float>,
        offsetX -> Nullable<Float>,
        offsetY -> Nullable<Float>,
        offsetZ -> Nullable<Float>,
        fReputationSizeMultiplier -> Nullable<Float>,
        fImaginationCost -> Nullable<Float>,
    }
}

table! {
    Factions (efId) {
        efId -> Integer,
        faction -> Nullable<Integer>,
        factionList -> Nullable<Text>,
        factionListFriendly -> Nullable<Integer>,
        friendList -> Nullable<Text>,
        enemyList -> Nullable<Text>,
    }
}

table! {
    FeatureGating (efId) {
        efId -> Integer,
        featureName -> Nullable<Text>,
        major -> Nullable<Integer>,
        current -> Nullable<Integer>,
        minor -> Nullable<Integer>,
        description -> Nullable<Text>,
    }
}

table! {
    FlairTable (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        asset -> Nullable<Text>,
    }
}

table! {
    Icons (efId) {
        efId -> Integer,
        IconID -> Nullable<Integer>,
        IconPath -> Nullable<Text>,
        IconName -> Nullable<Text>,
    }
}

table! {
    InventoryComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        itemid -> Nullable<Integer>,
        count -> Nullable<Integer>,
        equip -> Nullable<Integer>,
    }
}

table! {
    ItemComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        equipLocation -> Nullable<Text>,
        baseValue -> Nullable<Integer>,
        isKitPiece -> Nullable<Integer>,
        rarity -> Nullable<Integer>,
        itemType -> Nullable<Integer>,
        itemInfo -> Nullable<Integer>,
        inLootTable -> Nullable<Integer>,
        inVendor -> Nullable<Integer>,
        isUnique -> Nullable<Integer>,
        isBOP -> Nullable<Integer>,
        isBOE -> Nullable<Integer>,
        reqFlagID -> Nullable<Integer>,
        reqSpecialtyID -> Nullable<Integer>,
        reqSpecRank -> Nullable<Integer>,
        reqAchievementID -> Nullable<Integer>,
        stackSize -> Nullable<Integer>,
        color1 -> Nullable<Integer>,
        decal -> Nullable<Integer>,
        offsetGroupID -> Nullable<Integer>,
        buildTypes -> Nullable<Integer>,
        reqPrecondition -> Nullable<Text>,
        animationFlag -> Nullable<Integer>,
        equipEffects -> Nullable<Integer>,
        readyForQA -> Nullable<Integer>,
        itemRating -> Nullable<Integer>,
        isTwoHanded -> Nullable<Integer>,
        minNumRequired -> Nullable<Integer>,
        delResIndex -> Nullable<Integer>,
        currencyLOT -> Nullable<Integer>,
        altCurrencyCost -> Nullable<Integer>,
        subItems -> Nullable<Text>,
        audioEventUse -> Nullable<Text>,
        noEquipAnimation -> Nullable<Integer>,
        commendationLOT -> Nullable<Integer>,
        commendationCost -> Nullable<Integer>,
        audioEquipMetaEventSet -> Nullable<Text>,
        currencyCosts -> Nullable<Text>,
        ingredientInfo -> Nullable<Text>,
        locStatus -> Nullable<Integer>,
        forgeType -> Nullable<Integer>,
        SellMultiplier -> Nullable<Float>,
    }
}

table! {
    ItemEggData (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        chassie_type_id -> Nullable<Integer>,
    }
}

table! {
    ItemFoodData (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        element_1 -> Nullable<Integer>,
        element_1_amount -> Nullable<Integer>,
        element_2 -> Nullable<Integer>,
        element_2_amount -> Nullable<Integer>,
        element_3 -> Nullable<Integer>,
        element_3_amount -> Nullable<Integer>,
        element_4 -> Nullable<Integer>,
        element_4_amount -> Nullable<Integer>,
    }
}

table! {
    ItemSetSkills (efId) {
        efId -> Integer,
        SkillSetID -> Nullable<Integer>,
        SkillID -> Nullable<Integer>,
        SkillCastType -> Nullable<Integer>,
    }
}

table! {
    ItemSets (efId) {
        efId -> Integer,
        setID -> Nullable<Integer>,
        locStatus -> Nullable<Integer>,
        itemIDs -> Nullable<Text>,
        kitType -> Nullable<Integer>,
        kitRank -> Nullable<Integer>,
        kitImage -> Nullable<Integer>,
        skillSetWith2 -> Nullable<Integer>,
        skillSetWith3 -> Nullable<Integer>,
        skillSetWith4 -> Nullable<Integer>,
        skillSetWith5 -> Nullable<Integer>,
        skillSetWith6 -> Nullable<Integer>,
        localize -> Nullable<Integer>,
        gate_version -> Nullable<Text>,
        kitID -> Nullable<Integer>,
        priority -> Nullable<Float>,
    }
}

table! {
    JetPackPadComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        xDistance -> Nullable<Float>,
        yDistance -> Nullable<Float>,
        warnDistance -> Nullable<Float>,
        lotBlocker -> Nullable<Integer>,
        lotWarningVolume -> Nullable<Integer>,
    }
}

table! {
    LUPExhibitComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        minXZ -> Nullable<Float>,
        maxXZ -> Nullable<Float>,
        maxY -> Nullable<Float>,
        offsetX -> Nullable<Float>,
        offsetY -> Nullable<Float>,
        offsetZ -> Nullable<Float>,
    }
}

table! {
    LUPExhibitModelData (efId) {
        efId -> Integer,
        LOT -> Nullable<Integer>,
        minXZ -> Nullable<Float>,
        maxXZ -> Nullable<Float>,
        maxY -> Nullable<Float>,
        description -> Nullable<Text>,
        owner -> Nullable<Text>,
    }
}

table! {
    LUPZoneIDs (efId) {
        efId -> Integer,
        zoneID -> Nullable<Integer>,
    }
}

table! {
    LanguageType (efId) {
        efId -> Integer,
        LanguageID -> Nullable<Integer>,
        LanguageDescription -> Nullable<Text>,
    }
}

table! {
    LevelProgressionLookup (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        requiredUScore -> Nullable<Integer>,
        BehaviorEffect -> Nullable<Text>,
    }
}

table! {
    LootMatrix (efId) {
        efId -> Integer,
        LootMatrixIndex -> Nullable<Integer>,
        LootTableIndex -> Nullable<Integer>,
        RarityTableIndex -> Nullable<Integer>,
        percent -> Nullable<Float>,
        minToDrop -> Nullable<Integer>,
        maxToDrop -> Nullable<Integer>,
        id -> Nullable<Integer>,
        flagID -> Nullable<Integer>,
        gate_version -> Nullable<Text>,
    }
}

table! {
    LootMatrixIndex (efId) {
        efId -> Integer,
        LootMatrixIndex -> Nullable<Integer>,
        inNpcEditor -> Nullable<Integer>,
    }
}

table! {
    LootTable (efId) {
        efId -> Integer,
        itemid -> Nullable<Integer>,
        LootTableIndex -> Nullable<Integer>,
        id -> Nullable<Integer>,
        MissionDrop -> Nullable<Integer>,
        sortPriority -> Nullable<Integer>,
    }
}

table! {
    LootTableIndex (efId) {
        efId -> Integer,
        LootTableIndex -> Nullable<Integer>,
    }
}

table! {
    MinifigComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        head -> Nullable<Integer>,
        chest -> Nullable<Integer>,
        legs -> Nullable<Integer>,
        hairstyle -> Nullable<Integer>,
        haircolor -> Nullable<Integer>,
        chestdecal -> Nullable<Integer>,
        headcolor -> Nullable<Integer>,
        lefthand -> Nullable<Integer>,
        righthand -> Nullable<Integer>,
        eyebrowstyle -> Nullable<Integer>,
        eyesstyle -> Nullable<Integer>,
        mouthstyle -> Nullable<Integer>,
    }
}

table! {
    MinifigDecals_Eyebrows (efId) {
        efId -> Integer,
        ID -> Nullable<Integer>,
        High_path -> Nullable<Text>,
        Low_path -> Nullable<Text>,
        CharacterCreateValid -> Nullable<Integer>,
        male -> Nullable<Integer>,
        female -> Nullable<Integer>,
    }
}

table! {
    MinifigDecals_Eyes (efId) {
        efId -> Integer,
        ID -> Nullable<Integer>,
        High_path -> Nullable<Text>,
        Low_path -> Nullable<Text>,
        CharacterCreateValid -> Nullable<Integer>,
        male -> Nullable<Integer>,
        female -> Nullable<Integer>,
    }
}

table! {
    MinifigDecals_Legs (efId) {
        efId -> Integer,
        ID -> Nullable<Integer>,
        High_path -> Nullable<Text>,
    }
}

table! {
    MinifigDecals_Mouths (efId) {
        efId -> Integer,
        ID -> Nullable<Integer>,
        High_path -> Nullable<Text>,
        Low_path -> Nullable<Text>,
        CharacterCreateValid -> Nullable<Integer>,
        male -> Nullable<Integer>,
        female -> Nullable<Integer>,
    }
}

table! {
    MinifigDecals_Torsos (efId) {
        efId -> Integer,
        ID -> Nullable<Integer>,
        High_path -> Nullable<Text>,
        CharacterCreateValid -> Nullable<Integer>,
        male -> Nullable<Integer>,
        female -> Nullable<Integer>,
    }
}

table! {
    MissionEmail (efId) {
        efId -> Integer,
        ID -> Nullable<Integer>,
        messageType -> Nullable<Integer>,
        notificationGroup -> Nullable<Integer>,
        missionID -> Nullable<Integer>,
        attachmentLOT -> Nullable<Integer>,
        localize -> Nullable<Integer>,
        locStatus -> Nullable<Integer>,
        gate_version -> Nullable<Text>,
    }
}

table! {
    MissionNPCComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        missionID -> Nullable<Integer>,
        offersMission -> Nullable<Integer>,
        acceptsMission -> Nullable<Integer>,
        gate_version -> Nullable<Text>,
    }
}

table! {
    MissionTasks (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        locStatus -> Nullable<Integer>,
        taskType -> Nullable<Integer>,
        target -> Nullable<Integer>,
        targetGroup -> Nullable<Text>,
        targetValue -> Nullable<Integer>,
        taskParam1 -> Nullable<Text>,
        largeTaskIcon -> Nullable<Text>,
        IconID -> Nullable<Integer>,
        uid -> Nullable<Integer>,
        largeTaskIconID -> Nullable<Integer>,
        localize -> Nullable<Integer>,
        gate_version -> Nullable<Text>,
    }
}

table! {
    MissionText (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        story_icon -> Nullable<Text>,
        missionIcon -> Nullable<Text>,
        offerNPCIcon -> Nullable<Text>,
        IconID -> Nullable<Integer>,
        state_1_anim -> Nullable<Text>,
        state_2_anim -> Nullable<Text>,
        state_3_anim -> Nullable<Text>,
        state_4_anim -> Nullable<Text>,
        state_3_turnin_anim -> Nullable<Text>,
        state_4_turnin_anim -> Nullable<Text>,
        onclick_anim -> Nullable<Text>,
        CinematicAccepted -> Nullable<Text>,
        CinematicAcceptedLeadin -> Nullable<Float>,
        CinematicCompleted -> Nullable<Text>,
        CinematicCompletedLeadin -> Nullable<Float>,
        CinematicRepeatable -> Nullable<Text>,
        CinematicRepeatableLeadin -> Nullable<Float>,
        CinematicRepeatableCompleted -> Nullable<Text>,
        CinematicRepeatableCompletedLeadin -> Nullable<Float>,
        AudioEventGUID_Interact -> Nullable<Text>,
        AudioEventGUID_OfferAccept -> Nullable<Text>,
        AudioEventGUID_OfferDeny -> Nullable<Text>,
        AudioEventGUID_Completed -> Nullable<Text>,
        AudioEventGUID_TurnIn -> Nullable<Text>,
        AudioEventGUID_Failed -> Nullable<Text>,
        AudioEventGUID_Progress -> Nullable<Text>,
        AudioMusicCue_OfferAccept -> Nullable<Text>,
        AudioMusicCue_TurnIn -> Nullable<Text>,
        turnInIconID -> Nullable<Integer>,
        localize -> Nullable<Integer>,
        locStatus -> Nullable<Integer>,
        gate_version -> Nullable<Text>,
    }
}

table! {
    Missions (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        defined_type -> Nullable<Text>,
        defined_subtype -> Nullable<Text>,
        UISortOrder -> Nullable<Integer>,
        offer_objectID -> Nullable<Integer>,
        target_objectID -> Nullable<Integer>,
        reward_currency -> Nullable<Integer>,
        LegoScore -> Nullable<Integer>,
        reward_reputation -> Nullable<Integer>,
        isChoiceReward -> Nullable<Integer>,
        reward_item1 -> Nullable<Integer>,
        reward_item1_count -> Nullable<Integer>,
        reward_item2 -> Nullable<Integer>,
        reward_item2_count -> Nullable<Integer>,
        reward_item3 -> Nullable<Integer>,
        reward_item3_count -> Nullable<Integer>,
        reward_item4 -> Nullable<Integer>,
        reward_item4_count -> Nullable<Integer>,
        reward_emote -> Nullable<Integer>,
        reward_emote2 -> Nullable<Integer>,
        reward_emote3 -> Nullable<Integer>,
        reward_emote4 -> Nullable<Integer>,
        reward_maximagination -> Nullable<Integer>,
        reward_maxhealth -> Nullable<Integer>,
        reward_maxinventory -> Nullable<Integer>,
        reward_maxmodel -> Nullable<Integer>,
        reward_maxwidget -> Nullable<Integer>,
        reward_maxwallet -> Nullable<Integer>,
        repeatable -> Nullable<Integer>,
        reward_currency_repeatable -> Nullable<Integer>,
        reward_item1_repeatable -> Nullable<Integer>,
        reward_item1_repeat_count -> Nullable<Integer>,
        reward_item2_repeatable -> Nullable<Integer>,
        reward_item2_repeat_count -> Nullable<Integer>,
        reward_item3_repeatable -> Nullable<Integer>,
        reward_item3_repeat_count -> Nullable<Integer>,
        reward_item4_repeatable -> Nullable<Integer>,
        reward_item4_repeat_count -> Nullable<Integer>,
        time_limit -> Nullable<Integer>,
        isMission -> Nullable<Integer>,
        missionIconID -> Nullable<Integer>,
        prereqMissionID -> Nullable<Text>,
        localize -> Nullable<Integer>,
        inMOTD -> Nullable<Integer>,
        cooldownTime -> Nullable<Integer>,
        isRandom -> Nullable<Integer>,
        randomPool -> Nullable<Text>,
        UIPrereqID -> Nullable<Integer>,
        gate_version -> Nullable<Text>,
        HUDStates -> Nullable<Text>,
        locStatus -> Nullable<Integer>,
        reward_bankinventory -> Nullable<Integer>,
    }
}

table! {
    ModelBehavior (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        definitionXMLfilename -> Nullable<Text>,
    }
}

table! {
    ModularBuildComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        buildType -> Nullable<Integer>,
        xml -> Nullable<Text>,
        createdLOT -> Nullable<Integer>,
        createdPhysicsID -> Nullable<Integer>,
        AudioEventGUID_Snap -> Nullable<Text>,
        AudioEventGUID_Complete -> Nullable<Text>,
        AudioEventGUID_Present -> Nullable<Text>,
    }
}

table! {
    ModuleComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        partCode -> Nullable<Integer>,
        buildType -> Nullable<Integer>,
        xml -> Nullable<Text>,
        primarySoundGUID -> Nullable<Text>,
        assembledEffectID -> Nullable<Integer>,
    }
}

table! {
    MotionFX (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        typeID -> Nullable<Integer>,
        slamVelocity -> Nullable<Float>,
        addVelocity -> Nullable<Float>,
        duration -> Nullable<Float>,
        destGroupName -> Nullable<Text>,
        startScale -> Nullable<Float>,
        endScale -> Nullable<Float>,
        velocity -> Nullable<Float>,
        distance -> Nullable<Float>,
    }
}

table! {
    MovementAIComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        MovementType -> Nullable<Text>,
        WanderChance -> Nullable<Float>,
        WanderDelayMin -> Nullable<Float>,
        WanderDelayMax -> Nullable<Float>,
        WanderSpeed -> Nullable<Float>,
        WanderRadius -> Nullable<Float>,
        attachedPath -> Nullable<Text>,
    }
}

table! {
    MovingPlatforms (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        platformIsSimpleMover -> Nullable<Integer>,
        platformMoveX -> Nullable<Float>,
        platformMoveY -> Nullable<Float>,
        platformMoveZ -> Nullable<Float>,
        platformMoveTime -> Nullable<Float>,
        platformStartAtEnd -> Nullable<Integer>,
        description -> Nullable<Text>,
    }
}

table! {
    NpcIcons (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        color -> Nullable<Integer>,
        offset -> Nullable<Float>,
        LOT -> Nullable<Integer>,
        Texture -> Nullable<Text>,
        isClickable -> Nullable<Integer>,
        scale -> Nullable<Float>,
        rotateToFace -> Nullable<Integer>,
        compositeHorizOffset -> Nullable<Float>,
        compositeVertOffset -> Nullable<Float>,
        compositeScale -> Nullable<Float>,
        compositeConnectionNode -> Nullable<Text>,
        compositeLOTMultiMission -> Nullable<Integer>,
        compositeLOTMultiMissionVentor -> Nullable<Integer>,
        compositeIconTexture -> Nullable<Text>,
    }
}

table! {
    ObjectBehaviorXREF (efId) {
        efId -> Integer,
        LOT -> Nullable<Integer>,
        behaviorID1 -> Nullable<Integer>,
        behaviorID2 -> Nullable<Integer>,
        behaviorID3 -> Nullable<Integer>,
        behaviorID4 -> Nullable<Integer>,
        behaviorID5 -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Nullable<Integer>,
    }
}

table! {
    ObjectBehaviors (efId) {
        efId -> Integer,
        BehaviorID -> Nullable<Integer>,
        xmldata -> Nullable<Text>,
    }
}

table! {
    ObjectSkills (efId) {
        efId -> Integer,
        objectTemplate -> Nullable<Integer>,
        skillID -> Nullable<Integer>,
        castOnType -> Nullable<Integer>,
        AICombatWeight -> Nullable<Integer>,
    }
}

table! {
    Objects (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        placeable -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        description -> Nullable<Text>,
        localize -> Nullable<Integer>,
        npcTemplateID -> Nullable<Integer>,
        displayName -> Nullable<Text>,
        interactionDistance -> Nullable<Float>,
        nametag -> Nullable<Integer>,
        _internalNotes -> Nullable<Text>,
        locStatus -> Nullable<Integer>,
        gate_version -> Nullable<Text>,
        HQ_valid -> Nullable<Integer>,
    }
}

table! {
    PackageComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        LootMatrixIndex -> Nullable<Integer>,
        packageType -> Nullable<Integer>,
    }
}

table! {
    PetAbilities (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        AbilityName -> Nullable<Text>,
        ImaginationCost -> Nullable<Integer>,
        locStatus -> Nullable<Integer>,
    }
}

table! {
    PetComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        minTameUpdateTime -> Nullable<Float>,
        maxTameUpdateTime -> Nullable<Float>,
        percentTameChance -> Nullable<Float>,
        tamability -> Nullable<Float>,
        elementType -> Nullable<Integer>,
        walkSpeed -> Nullable<Float>,
        runSpeed -> Nullable<Float>,
        sprintSpeed -> Nullable<Float>,
        idleTimeMin -> Nullable<Float>,
        idleTimeMax -> Nullable<Float>,
        petForm -> Nullable<Integer>,
        imaginationDrainRate -> Nullable<Float>,
        AudioMetaEventSet -> Nullable<Text>,
        buffIDs -> Nullable<Text>,
    }
}

table! {
    PetNestComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        ElementalType -> Nullable<Integer>,
    }
}

table! {
    PhysicsComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        #[sql_name = "static"]
        static_ -> Nullable<Float>,
        physics_asset -> Nullable<Text>,
        jump -> Nullable<Float>,
        doublejump -> Nullable<Float>,
        speed -> Nullable<Float>,
        rotSpeed -> Nullable<Float>,
        playerHeight -> Nullable<Float>,
        playerRadius -> Nullable<Float>,
        pcShapeType -> Nullable<Integer>,
        collisionGroup -> Nullable<Integer>,
        airSpeed -> Nullable<Float>,
        boundaryAsset -> Nullable<Text>,
        jumpAirSpeed -> Nullable<Float>,
        friction -> Nullable<Float>,
        gravityVolumeAsset -> Nullable<Text>,
    }
}

table! {
    PlayerFlags (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        SessionOnly -> Nullable<Integer>,
        OnlySetByServer -> Nullable<Integer>,
        SessionZoneOnly -> Nullable<Integer>,
    }
}

table! {
    PlayerStatistics (efId) {
        efId -> Integer,
        statID -> Nullable<Integer>,
        sortOrder -> Nullable<Integer>,
        locStatus -> Nullable<Integer>,
        gate_version -> Nullable<Text>,
    }
}

table! {
    PossessableComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        controlSchemeID -> Nullable<Integer>,
        minifigAttachPoint -> Nullable<Text>,
        minifigAttachAnimation -> Nullable<Text>,
        minifigDetachAnimation -> Nullable<Text>,
        mountAttachAnimation -> Nullable<Text>,
        mountDetachAnimation -> Nullable<Text>,
        attachOffsetFwd -> Nullable<Float>,
        attachOffsetRight -> Nullable<Float>,
        possessionType -> Nullable<Integer>,
        wantBillboard -> Nullable<Integer>,
        billboardOffsetUp -> Nullable<Float>,
        depossessOnHit -> Nullable<Integer>,
        hitStunTime -> Nullable<Float>,
        skillSet -> Nullable<Integer>,
    }
}

table! {
    Preconditions (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Nullable<Integer>,
        targetLOT -> Nullable<Text>,
        targetGroup -> Nullable<Text>,
        targetCount -> Nullable<Integer>,
        iconID -> Nullable<Integer>,
        localize -> Nullable<Integer>,
        validContexts -> Nullable<Integer>,
        locStatus -> Nullable<Integer>,
        gate_version -> Nullable<Text>,
    }
}

table! {
    PropertyEntranceComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        mapID -> Nullable<Integer>,
        propertyName -> Nullable<Text>,
        isOnProperty -> Nullable<Integer>,
        groupType -> Nullable<Text>,
    }
}

table! {
    PropertyTemplate (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        mapID -> Nullable<Integer>,
        vendorMapID -> Nullable<Integer>,
        spawnName -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Nullable<Integer>,
        sizecode -> Nullable<Integer>,
        minimumPrice -> Nullable<Integer>,
        rentDuration -> Nullable<Integer>,
        path -> Nullable<Text>,
        cloneLimit -> Nullable<Integer>,
        durationType -> Nullable<Integer>,
        achievementRequired -> Nullable<Integer>,
        zoneX -> Nullable<Float>,
        zoneY -> Nullable<Float>,
        zoneZ -> Nullable<Float>,
        maxBuildHeight -> Nullable<Float>,
        localize -> Nullable<Integer>,
        reputationPerMinute -> Nullable<Integer>,
        locStatus -> Nullable<Integer>,
        gate_version -> Nullable<Text>,
    }
}

table! {
    ProximityMonitorComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        Proximities -> Nullable<Text>,
        LoadOnClient -> Nullable<Integer>,
        LoadOnServer -> Nullable<Integer>,
    }
}

table! {
    ProximityTypes (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        Name -> Nullable<Text>,
        Radius -> Nullable<Integer>,
        CollisionGroup -> Nullable<Integer>,
        PassiveChecks -> Nullable<Integer>,
        IconID -> Nullable<Integer>,
        LoadOnClient -> Nullable<Integer>,
        LoadOnServer -> Nullable<Integer>,
    }
}

table! {
    RacingModuleComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        topSpeed -> Nullable<Float>,
        acceleration -> Nullable<Float>,
        handling -> Nullable<Float>,
        stability -> Nullable<Float>,
        imagination -> Nullable<Float>,
    }
}

table! {
    RailActivatorComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        startAnim -> Nullable<Text>,
        loopAnim -> Nullable<Text>,
        stopAnim -> Nullable<Text>,
        startSound -> Nullable<Text>,
        loopSound -> Nullable<Text>,
        stopSound -> Nullable<Text>,
        effectIDs -> Nullable<Text>,
        preconditions -> Nullable<Text>,
        playerCollision -> Nullable<Integer>,
        cameraLocked -> Nullable<Integer>,
        StartEffectID -> Nullable<Text>,
        StopEffectID -> Nullable<Text>,
        DamageImmune -> Nullable<Integer>,
        NoAggro -> Nullable<Integer>,
        ShowNameBillboard -> Nullable<Integer>,
    }
}

table! {
    RarityTable (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        randmax -> Nullable<Float>,
        rarity -> Nullable<Integer>,
        RarityTableIndex -> Nullable<Integer>,
    }
}

table! {
    RarityTableIndex (efId) {
        efId -> Integer,
        RarityTableIndex -> Nullable<Integer>,
    }
}

table! {
    RebuildComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        reset_time -> Nullable<Float>,
        complete_time -> Nullable<Float>,
        take_imagination -> Nullable<Integer>,
        interruptible -> Nullable<Integer>,
        self_activator -> Nullable<Integer>,
        custom_modules -> Nullable<Text>,
        activityID -> Nullable<Integer>,
        post_imagination_cost -> Nullable<Integer>,
        time_before_smash -> Nullable<Float>,
    }
}

table! {
    RebuildSections (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        rebuildID -> Nullable<Integer>,
        objectID -> Nullable<Integer>,
        offset_x -> Nullable<Float>,
        offset_y -> Nullable<Float>,
        offset_z -> Nullable<Float>,
        fall_angle_x -> Nullable<Float>,
        fall_angle_y -> Nullable<Float>,
        fall_angle_z -> Nullable<Float>,
        fall_height -> Nullable<Float>,
        requires_list -> Nullable<Text>,
        size -> Nullable<Integer>,
        bPlaced -> Nullable<Integer>,
    }
}

table! {
    Release_Version (efId) {
        efId -> Integer,
        ReleaseVersion -> Nullable<Text>,
        ReleaseDate -> Nullable<Integer>,
    }
}

table! {
    RenderComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        render_asset -> Nullable<Text>,
        icon_asset -> Nullable<Text>,
        IconID -> Nullable<Integer>,
        shader_id -> Nullable<Integer>,
        effect1 -> Nullable<Integer>,
        effect2 -> Nullable<Integer>,
        effect3 -> Nullable<Integer>,
        effect4 -> Nullable<Integer>,
        effect5 -> Nullable<Integer>,
        effect6 -> Nullable<Integer>,
        animationGroupIDs -> Nullable<Text>,
        fade -> Nullable<Integer>,
        usedropshadow -> Nullable<Integer>,
        preloadAnimations -> Nullable<Integer>,
        fadeInTime -> Nullable<Float>,
        maxShadowDistance -> Nullable<Float>,
        ignoreCameraCollision -> Nullable<Integer>,
        renderComponentLOD1 -> Nullable<Integer>,
        renderComponentLOD2 -> Nullable<Integer>,
        gradualSnap -> Nullable<Integer>,
        animationFlag -> Nullable<Integer>,
        AudioMetaEventSet -> Nullable<Text>,
        billboardHeight -> Nullable<Float>,
        chatBubbleOffset -> Nullable<Float>,
        staticBillboard -> Nullable<Integer>,
        LXFMLFolder -> Nullable<Text>,
        attachIndicatorsToNode -> Nullable<Integer>,
    }
}

table! {
    RenderComponentFlash (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        interactive -> Nullable<Integer>,
        animated -> Nullable<Integer>,
        nodeName -> Nullable<Text>,
        flashPath -> Nullable<Text>,
        elementName -> Nullable<Text>,
        _uid -> Nullable<Integer>,
    }
}

table! {
    RenderComponentWrapper (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        defaultWrapperAsset -> Nullable<Text>,
    }
}

table! {
    RenderIconAssets (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        icon_asset -> Nullable<Text>,
        blank_column -> Nullable<Text>,
    }
}

table! {
    ReputationRewards (efId) {
        efId -> Integer,
        repLevel -> Nullable<Integer>,
        sublevel -> Nullable<Integer>,
        reputation -> Nullable<Float>,
    }
}

table! {
    RewardCodes (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        code -> Nullable<Text>,
        attachmentLOT -> Nullable<Integer>,
        locStatus -> Nullable<Integer>,
        gate_version -> Nullable<Text>,
    }
}

table! {
    Rewards (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        LevelID -> Nullable<Integer>,
        MissionID -> Nullable<Integer>,
        RewardType -> Nullable<Integer>,
        value -> Nullable<Integer>,
        count -> Nullable<Integer>,
    }
}

table! {
    RocketLaunchpadControlComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        targetZone -> Nullable<Integer>,
        defaultZoneID -> Nullable<Integer>,
        targetScene -> Nullable<Text>,
        gmLevel -> Nullable<Integer>,
        playerAnimation -> Nullable<Text>,
        rocketAnimation -> Nullable<Text>,
        launchMusic -> Nullable<Text>,
        useLaunchPrecondition -> Nullable<Integer>,
        useAltLandingPrecondition -> Nullable<Integer>,
        launchPrecondition -> Nullable<Text>,
        altLandingPrecondition -> Nullable<Text>,
        altLandingSpawnPointName -> Nullable<Text>,
    }
}

table! {
    SceneTable (efId) {
        efId -> Integer,
        sceneID -> Nullable<Integer>,
        sceneName -> Nullable<Text>,
    }
}

table! {
    ScriptComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        script_name -> Nullable<Text>,
        client_script_name -> Nullable<Text>,
    }
}

table! {
    SkillBehavior (efId) {
        efId -> Integer,
        skillID -> Nullable<Integer>,
        locStatus -> Nullable<Integer>,
        behaviorID -> Nullable<Integer>,
        imaginationcost -> Nullable<Integer>,
        cooldowngroup -> Nullable<Integer>,
        cooldown -> Nullable<Float>,
        inNpcEditor -> Nullable<Integer>,
        skillIcon -> Nullable<Integer>,
        oomSkillID -> Nullable<Text>,
        oomBehaviorEffectID -> Nullable<Integer>,
        castTypeDesc -> Nullable<Integer>,
        imBonusUI -> Nullable<Integer>,
        lifeBonusUI -> Nullable<Integer>,
        armorBonusUI -> Nullable<Integer>,
        damageUI -> Nullable<Integer>,
        hideIcon -> Nullable<Integer>,
        localize -> Nullable<Integer>,
        gate_version -> Nullable<Text>,
        cancelType -> Nullable<Integer>,
    }
}

table! {
    SmashableChain (efId) {
        efId -> Integer,
        chainIndex -> Nullable<Integer>,
        chainLevel -> Nullable<Integer>,
        lootMatrixID -> Nullable<Integer>,
        rarityTableIndex -> Nullable<Integer>,
        currencyIndex -> Nullable<Integer>,
        currencyLevel -> Nullable<Integer>,
        smashCount -> Nullable<Integer>,
        timeLimit -> Nullable<Integer>,
        chainStepID -> Nullable<Integer>,
    }
}

table! {
    SmashableChainIndex (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        targetGroup -> Nullable<Text>,
        description -> Nullable<Text>,
        continuous -> Nullable<Integer>,
    }
}

table! {
    SmashableComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        LootMatrixIndex -> Nullable<Integer>,
    }
}

table! {
    SmashableElements (efId) {
        efId -> Integer,
        elementID -> Nullable<Integer>,
        dropWeight -> Nullable<Integer>,
    }
}

table! {
    SpeedchatMenu (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        parentId -> Nullable<Integer>,
        emoteId -> Nullable<Integer>,
        imageName -> Nullable<Text>,
        localize -> Nullable<Integer>,
        locStatus -> Nullable<Integer>,
        gate_version -> Nullable<Text>,
    }
}

table! {
    SubscriptionPricing (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        countryCode -> Nullable<Text>,
        monthlyFeeGold -> Nullable<Text>,
        monthlyFeeSilver -> Nullable<Text>,
        monthlyFeeBronze -> Nullable<Text>,
        monetarySymbol -> Nullable<Integer>,
        symbolIsAppended -> Nullable<Integer>,
    }
}

table! {
    SurfaceType (efId) {
        efId -> Integer,
        SurfaceType -> Nullable<Integer>,
        FootstepNDAudioMetaEventSetName -> Nullable<Text>,
    }
}

table! {
    TamingBuildPuzzles (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        PuzzleModelLot -> Nullable<Integer>,
        NPCLot -> Nullable<Integer>,
        ValidPiecesLXF -> Nullable<Text>,
        InvalidPiecesLXF -> Nullable<Text>,
        Difficulty -> Nullable<Integer>,
        Timelimit -> Nullable<Integer>,
        NumValidPieces -> Nullable<Integer>,
        TotalNumPieces -> Nullable<Integer>,
        ModelName -> Nullable<Text>,
        FullModelLXF -> Nullable<Text>,
        Duration -> Nullable<Float>,
        imagCostPerBuild -> Nullable<Integer>,
    }
}

table! {
    TextDescription (efId) {
        efId -> Integer,
        TextID -> Nullable<Integer>,
        TestDescription -> Nullable<Text>,
    }
}

table! {
    TextLanguage (efId) {
        efId -> Integer,
        TextID -> Nullable<Integer>,
        LanguageID -> Nullable<Integer>,
        Text -> Nullable<Text>,
    }
}

table! {
    TrailEffects (efId) {
        efId -> Integer,
        trailID -> Nullable<Integer>,
        textureName -> Nullable<Text>,
        blendmode -> Nullable<Integer>,
        cardlifetime -> Nullable<Float>,
        colorlifetime -> Nullable<Float>,
        minTailFade -> Nullable<Float>,
        tailFade -> Nullable<Float>,
        max_particles -> Nullable<Integer>,
        birthDelay -> Nullable<Float>,
        deathDelay -> Nullable<Float>,
        bone1 -> Nullable<Text>,
        bone2 -> Nullable<Text>,
        texLength -> Nullable<Float>,
        texWidth -> Nullable<Float>,
        startColorR -> Nullable<Float>,
        startColorG -> Nullable<Float>,
        startColorB -> Nullable<Float>,
        startColorA -> Nullable<Float>,
        middleColorR -> Nullable<Float>,
        middleColorG -> Nullable<Float>,
        middleColorB -> Nullable<Float>,
        middleColorA -> Nullable<Float>,
        endColorR -> Nullable<Float>,
        endColorG -> Nullable<Float>,
        endColorB -> Nullable<Float>,
        endColorA -> Nullable<Float>,
    }
}

table! {
    UGBehaviorSounds (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        guid -> Nullable<Text>,
        localize -> Nullable<Integer>,
        locStatus -> Nullable<Integer>,
        gate_version -> Nullable<Text>,
    }
}

table! {
    VehiclePhysics (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        hkxFilename -> Nullable<Text>,
        fGravityScale -> Nullable<Float>,
        fMass -> Nullable<Float>,
        fChassisFriction -> Nullable<Float>,
        fMaxSpeed -> Nullable<Float>,
        fEngineTorque -> Nullable<Float>,
        fBrakeFrontTorque -> Nullable<Float>,
        fBrakeRearTorque -> Nullable<Float>,
        fBrakeMinInputToBlock -> Nullable<Float>,
        fBrakeMinTimeToBlock -> Nullable<Float>,
        fSteeringMaxAngle -> Nullable<Float>,
        fSteeringSpeedLimitForMaxAngle -> Nullable<Float>,
        fSteeringMinAngle -> Nullable<Float>,
        fFwdBias -> Nullable<Float>,
        fFrontTireFriction -> Nullable<Float>,
        fRearTireFriction -> Nullable<Float>,
        fFrontTireFrictionSlide -> Nullable<Float>,
        fRearTireFrictionSlide -> Nullable<Float>,
        fFrontTireSlipAngle -> Nullable<Float>,
        fRearTireSlipAngle -> Nullable<Float>,
        fWheelWidth -> Nullable<Float>,
        fWheelRadius -> Nullable<Float>,
        fWheelMass -> Nullable<Float>,
        fReorientPitchStrength -> Nullable<Float>,
        fReorientRollStrength -> Nullable<Float>,
        fSuspensionLength -> Nullable<Float>,
        fSuspensionStrength -> Nullable<Float>,
        fSuspensionDampingCompression -> Nullable<Float>,
        fSuspensionDampingRelaxation -> Nullable<Float>,
        iChassisCollisionGroup -> Nullable<Integer>,
        fNormalSpinDamping -> Nullable<Float>,
        fCollisionSpinDamping -> Nullable<Float>,
        fCollisionThreshold -> Nullable<Float>,
        fTorqueRollFactor -> Nullable<Float>,
        fTorquePitchFactor -> Nullable<Float>,
        fTorqueYawFactor -> Nullable<Float>,
        fInertiaRoll -> Nullable<Float>,
        fInertiaPitch -> Nullable<Float>,
        fInertiaYaw -> Nullable<Float>,
        fExtraTorqueFactor -> Nullable<Float>,
        fCenterOfMassFwd -> Nullable<Float>,
        fCenterOfMassUp -> Nullable<Float>,
        fCenterOfMassRight -> Nullable<Float>,
        fWheelHardpointFrontFwd -> Nullable<Float>,
        fWheelHardpointFrontUp -> Nullable<Float>,
        fWheelHardpointFrontRight -> Nullable<Float>,
        fWheelHardpointRearFwd -> Nullable<Float>,
        fWheelHardpointRearUp -> Nullable<Float>,
        fWheelHardpointRearRight -> Nullable<Float>,
        fInputTurnSpeed -> Nullable<Float>,
        fInputDeadTurnBackSpeed -> Nullable<Float>,
        fInputAccelSpeed -> Nullable<Float>,
        fInputDeadAccelDownSpeed -> Nullable<Float>,
        fInputDecelSpeed -> Nullable<Float>,
        fInputDeadDecelDownSpeed -> Nullable<Float>,
        fInputSlopeChangePointX -> Nullable<Float>,
        fInputInitialSlope -> Nullable<Float>,
        fInputDeadZone -> Nullable<Float>,
        fAeroAirDensity -> Nullable<Float>,
        fAeroFrontalArea -> Nullable<Float>,
        fAeroDragCoefficient -> Nullable<Float>,
        fAeroLiftCoefficient -> Nullable<Float>,
        fAeroExtraGravity -> Nullable<Float>,
        fBoostTopSpeed -> Nullable<Float>,
        fBoostCostPerSecond -> Nullable<Float>,
        fBoostAccelerateChange -> Nullable<Float>,
        fBoostDampingChange -> Nullable<Float>,
        fPowerslideNeutralAngle -> Nullable<Float>,
        fPowerslideTorqueStrength -> Nullable<Float>,
        iPowerslideNumTorqueApplications -> Nullable<Integer>,
        fImaginationTankSize -> Nullable<Float>,
        fSkillCost -> Nullable<Float>,
        fWreckSpeedBase -> Nullable<Float>,
        fWreckSpeedPercent -> Nullable<Float>,
        fWreckMinAngle -> Nullable<Float>,
        AudioEventEngine -> Nullable<Text>,
        AudioEventSkid -> Nullable<Text>,
        AudioEventLightHit -> Nullable<Text>,
        AudioSpeedThresholdLightHit -> Nullable<Float>,
        AudioTimeoutLightHit -> Nullable<Float>,
        AudioEventHeavyHit -> Nullable<Text>,
        AudioSpeedThresholdHeavyHit -> Nullable<Float>,
        AudioTimeoutHeavyHit -> Nullable<Float>,
        AudioEventStart -> Nullable<Text>,
        AudioEventTreadConcrete -> Nullable<Text>,
        AudioEventTreadSand -> Nullable<Text>,
        AudioEventTreadWood -> Nullable<Text>,
        AudioEventTreadDirt -> Nullable<Text>,
        AudioEventTreadPlastic -> Nullable<Text>,
        AudioEventTreadGrass -> Nullable<Text>,
        AudioEventTreadGravel -> Nullable<Text>,
        AudioEventTreadMud -> Nullable<Text>,
        AudioEventTreadWater -> Nullable<Text>,
        AudioEventTreadSnow -> Nullable<Text>,
        AudioEventTreadIce -> Nullable<Text>,
        AudioEventTreadMetal -> Nullable<Text>,
        AudioEventTreadLeaves -> Nullable<Text>,
        AudioEventLightLand -> Nullable<Text>,
        AudioAirtimeForLightLand -> Nullable<Float>,
        AudioEventHeavyLand -> Nullable<Text>,
        AudioAirtimeForHeavyLand -> Nullable<Float>,
        bWheelsVisible -> Nullable<Integer>,
    }
}

table! {
    VehicleStatMap (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        ModuleStat -> Nullable<Text>,
        HavokStat -> Nullable<Text>,
        HavokChangePerModuleStat -> Nullable<Float>,
    }
}

table! {
    VendorComponent (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        buyScalar -> Nullable<Float>,
        sellScalar -> Nullable<Float>,
        refreshTimeSeconds -> Nullable<Float>,
        LootMatrixIndex -> Nullable<Integer>,
    }
}

table! {
    WhatsCoolItemSpotlight (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        itemID -> Nullable<Integer>,
        localize -> Nullable<Integer>,
        gate_version -> Nullable<Text>,
        locStatus -> Nullable<Integer>,
    }
}

table! {
    WhatsCoolNewsAndTips (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        iconID -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Nullable<Integer>,
        localize -> Nullable<Integer>,
        gate_version -> Nullable<Text>,
        locStatus -> Nullable<Integer>,
    }
}

table! {
    WorldConfig (efId) {
        efId -> Integer,
        WorldConfigID -> Nullable<Integer>,
        pegravityvalue -> Nullable<Float>,
        pebroadphaseworldsize -> Nullable<Float>,
        pegameobjscalefactor -> Nullable<Float>,
        character_rotation_speed -> Nullable<Float>,
        character_walk_forward_speed -> Nullable<Float>,
        character_walk_backward_speed -> Nullable<Float>,
        character_walk_strafe_speed -> Nullable<Float>,
        character_walk_strafe_forward_speed -> Nullable<Float>,
        character_walk_strafe_backward_speed -> Nullable<Float>,
        character_run_backward_speed -> Nullable<Float>,
        character_run_strafe_speed -> Nullable<Float>,
        character_run_strafe_forward_speed -> Nullable<Float>,
        character_run_strafe_backward_speed -> Nullable<Float>,
        global_cooldown -> Nullable<Float>,
        characterGroundedTime -> Nullable<Float>,
        characterGroundedSpeed -> Nullable<Float>,
        globalImmunityTime -> Nullable<Float>,
        character_max_slope -> Nullable<Float>,
        defaultrespawntime -> Nullable<Float>,
        mission_tooltip_timeout -> Nullable<Float>,
        vendor_buy_multiplier -> Nullable<Float>,
        pet_follow_radius -> Nullable<Float>,
        character_eye_height -> Nullable<Float>,
        flight_vertical_velocity -> Nullable<Float>,
        flight_airspeed -> Nullable<Float>,
        flight_fuel_ratio -> Nullable<Float>,
        flight_max_airspeed -> Nullable<Float>,
        fReputationPerVote -> Nullable<Float>,
        nPropertyCloneLimit -> Nullable<Integer>,
        defaultHomespaceTemplate -> Nullable<Integer>,
        coins_lost_on_death_percent -> Nullable<Float>,
        coins_lost_on_death_min -> Nullable<Integer>,
        coins_lost_on_death_max -> Nullable<Integer>,
        character_votes_per_day -> Nullable<Integer>,
        property_moderation_request_approval_cost -> Nullable<Integer>,
        property_moderation_request_review_cost -> Nullable<Integer>,
        propertyModRequestsAllowedSpike -> Nullable<Integer>,
        propertyModRequestsAllowedInterval -> Nullable<Integer>,
        propertyModRequestsAllowedTotal -> Nullable<Integer>,
        propertyModRequestsSpikeDuration -> Nullable<Integer>,
        propertyModRequestsIntervalDuration -> Nullable<Integer>,
        modelModerateOnCreate -> Nullable<Integer>,
        defaultPropertyMaxHeight -> Nullable<Float>,
        reputationPerVoteCast -> Nullable<Float>,
        reputationPerVoteReceived -> Nullable<Float>,
        showcaseTopModelConsiderationBattles -> Nullable<Integer>,
        reputationPerBattlePromotion -> Nullable<Float>,
        coins_lost_on_death_min_timeout -> Nullable<Float>,
        coins_lost_on_death_max_timeout -> Nullable<Float>,
        mail_base_fee -> Nullable<Integer>,
        mail_percent_attachment_fee -> Nullable<Float>,
        propertyReputationDelay -> Nullable<Integer>,
        LevelCap -> Nullable<Integer>,
        LevelUpBehaviorEffect -> Nullable<Text>,
        CharacterVersion -> Nullable<Integer>,
        LevelCapCurrencyConversion -> Nullable<Integer>,
    }
}

table! {
    ZoneLoadingTips (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        zoneid -> Nullable<Integer>,
        imagelocation -> Nullable<Text>,
        localize -> Nullable<Integer>,
        gate_version -> Nullable<Text>,
        locStatus -> Nullable<Integer>,
        weight -> Nullable<Integer>,
        targetVersion -> Nullable<Text>,
    }
}

table! {
    ZoneSummary (efId) {
        efId -> Integer,
        zoneID -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Nullable<Integer>,
        value -> Nullable<Integer>,
        _uniqueID -> Nullable<Integer>,
    }
}

table! {
    ZoneTable (efId) {
        efId -> Integer,
        zoneID -> Nullable<Integer>,
        locStatus -> Nullable<Integer>,
        zoneName -> Nullable<Text>,
        scriptID -> Nullable<Integer>,
        ghostdistance_min -> Nullable<Float>,
        ghostdistance -> Nullable<Float>,
        population_soft_cap -> Nullable<Integer>,
        population_hard_cap -> Nullable<Integer>,
        DisplayDescription -> Nullable<Text>,
        mapFolder -> Nullable<Text>,
        smashableMinDistance -> Nullable<Float>,
        smashableMaxDistance -> Nullable<Float>,
        mixerProgram -> Nullable<Text>,
        clientPhysicsFramerate -> Nullable<Text>,
        serverPhysicsFramerate -> Nullable<Text>,
        zoneControlTemplate -> Nullable<Integer>,
        widthInChunks -> Nullable<Integer>,
        heightInChunks -> Nullable<Integer>,
        petsAllowed -> Nullable<Integer>,
        localize -> Nullable<Integer>,
        fZoneWeight -> Nullable<Float>,
        thumbnail -> Nullable<Text>,
        PlayerLoseCoinsOnDeath -> Nullable<Integer>,
        disableSaveLoc -> Nullable<Integer>,
        teamRadius -> Nullable<Float>,
        gate_version -> Nullable<Text>,
        mountsAllowed -> Nullable<Integer>,
    }
}

table! {
    brickAttributes (efId) {
        efId -> Integer,
        ID -> Nullable<Integer>,
        icon_asset -> Nullable<Text>,
        display_order -> Nullable<Integer>,
        locStatus -> Nullable<Integer>,
    }
}

table! {
    dtproperties (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        objectid -> Nullable<Integer>,
        property -> Nullable<Text>,
        value -> Nullable<Text>,
        uvalue -> Nullable<Text>,
        lvalue -> Nullable<Integer>,
        version -> Nullable<Integer>,
    }
}

table! {
    mapAnimationPriorities (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        priority -> Nullable<Float>,
    }
}

table! {
    mapAssetType (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        label -> Nullable<Text>,
        pathdir -> Nullable<Text>,
        typelabel -> Nullable<Text>,
    }
}

table! {
    mapIcon (efId) {
        efId -> Integer,
        LOT -> Nullable<Integer>,
        iconID -> Nullable<Integer>,
        iconState -> Nullable<Integer>,
    }
}

table! {
    mapItemTypes (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        description -> Nullable<Text>,
        equipLocation -> Nullable<Text>,
    }
}

table! {
    mapRenderEffects (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        gameID -> Nullable<Integer>,
        description -> Nullable<Text>,
    }
}

table! {
    mapShaders (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        label -> Nullable<Text>,
        gameValue -> Nullable<Integer>,
        priority -> Nullable<Integer>,
    }
}

table! {
    mapTextureResource (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        texturepath -> Nullable<Text>,
        SurfaceType -> Nullable<Integer>,
    }
}

table! {
    map_BlueprintCategory (efId) {
        efId -> Integer,
        id -> Nullable<Integer>,
        description -> Nullable<Text>,
        enabled -> Nullable<Integer>,
    }
}

table! {
    sysdiagrams (efId) {
        efId -> Integer,
        name -> Nullable<Text>,
        principal_id -> Nullable<Integer>,
        diagram_id -> Nullable<Integer>,
        version -> Nullable<Integer>,
        definition -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    AICombatRoles,
    AccessoryDefaultLoc,
    Activities,
    ActivityRewards,
    ActivityText,
    AnimationIndex,
    Animations,
    BaseCombatAIComponent,
    BehaviorEffect,
    BehaviorParameter,
    BehaviorTemplate,
    BehaviorTemplateName,
    Blueprints,
    BrickColors,
    BrickIDTable,
    BuffDefinitions,
    BuffParameters,
    Camera,
    CelebrationParameters,
    ChoiceBuildComponent,
    CollectibleComponent,
    ComponentsRegistry,
    ControlSchemes,
    CurrencyDenominations,
    CurrencyTable,
    DBExclude,
    DeletionRestrictions,
    DestructibleComponent,
    DevModelBehaviors,
    Emotes,
    EventGating,
    ExhibitComponent,
    Factions,
    FeatureGating,
    FlairTable,
    Icons,
    InventoryComponent,
    ItemComponent,
    ItemEggData,
    ItemFoodData,
    ItemSetSkills,
    ItemSets,
    JetPackPadComponent,
    LUPExhibitComponent,
    LUPExhibitModelData,
    LUPZoneIDs,
    LanguageType,
    LevelProgressionLookup,
    LootMatrix,
    LootMatrixIndex,
    LootTable,
    LootTableIndex,
    MinifigComponent,
    MinifigDecals_Eyebrows,
    MinifigDecals_Eyes,
    MinifigDecals_Legs,
    MinifigDecals_Mouths,
    MinifigDecals_Torsos,
    MissionEmail,
    MissionNPCComponent,
    MissionTasks,
    MissionText,
    Missions,
    ModelBehavior,
    ModularBuildComponent,
    ModuleComponent,
    MotionFX,
    MovementAIComponent,
    MovingPlatforms,
    NpcIcons,
    ObjectBehaviorXREF,
    ObjectBehaviors,
    ObjectSkills,
    Objects,
    PackageComponent,
    PetAbilities,
    PetComponent,
    PetNestComponent,
    PhysicsComponent,
    PlayerFlags,
    PlayerStatistics,
    PossessableComponent,
    Preconditions,
    PropertyEntranceComponent,
    PropertyTemplate,
    ProximityMonitorComponent,
    ProximityTypes,
    RacingModuleComponent,
    RailActivatorComponent,
    RarityTable,
    RarityTableIndex,
    RebuildComponent,
    RebuildSections,
    Release_Version,
    RenderComponent,
    RenderComponentFlash,
    RenderComponentWrapper,
    RenderIconAssets,
    ReputationRewards,
    RewardCodes,
    Rewards,
    RocketLaunchpadControlComponent,
    SceneTable,
    ScriptComponent,
    SkillBehavior,
    SmashableChain,
    SmashableChainIndex,
    SmashableComponent,
    SmashableElements,
    SpeedchatMenu,
    SubscriptionPricing,
    SurfaceType,
    TamingBuildPuzzles,
    TextDescription,
    TextLanguage,
    TrailEffects,
    UGBehaviorSounds,
    VehiclePhysics,
    VehicleStatMap,
    VendorComponent,
    WhatsCoolItemSpotlight,
    WhatsCoolNewsAndTips,
    WorldConfig,
    ZoneLoadingTips,
    ZoneSummary,
    ZoneTable,
    brickAttributes,
    dtproperties,
    mapAnimationPriorities,
    mapAssetType,
    mapIcon,
    mapItemTypes,
    mapRenderEffects,
    mapShaders,
    mapTextureResource,
    map_BlueprintCategory,
    sysdiagrams,
);