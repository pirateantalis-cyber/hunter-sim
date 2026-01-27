# Contributing Your IRL Build

We need your help! Real player data is critical for validating our simulator. If you have an active build in CIFI, please submit it!

## Why Your Data Matters

Each build submission:
- ✅ Strengthens validation accuracy
- ✅ Helps us discover undiscovered game mechanics  
- ✅ Especially important for **Ozzy** (we currently have 1 data point!)
- ✅ Ensures the tool works for players like you
- ✅ Benefits the entire community

## What We Need

### Essential Information

For your hunter, provide:

1. **Hunter Details**
   - Hunter name: Borge / Ozzy / Knox
   - Current level
   - Max stage reached in-game

2. **Current Stats** (from your hunter's stat screen)
   - Total loot earned (or per resource: common, uncommon, rare)
   - Total XP earned
   - Current stage at which these stats were achieved

3. **Build Configuration**
   - All talent levels
   - All attribute point distribution
   - Relics owned and their levels
   - Inscryptions and levels
   - Gadgets and levels
   - Gems and levels
   - Construction milestone statuses

4. **Account Bonuses** (if applicable)
   - Global loot bonuses
   - Global XP bonuses
   - Ultima multiplier
   - Scavenger/Scavenger 2 levels

### Optional But Helpful

- Screenshot of your hunter's stats screen
- Screenshot of your build tree
- Notes on your playstyle (active/idle)
- Approximate time to reach current stage

## How to Submit

### Option 1: GitHub Issue (Easiest)

1. Go to [GitHub Issues](https://github.com/pirateantalis-cyber/hunter-sim/issues/new)

2. Create new issue with title: `[IRL-BUILD] Hunter Name - Level XX`

3. Use this template:

```markdown
## Build Submission

**Hunter:** [Borge/Ozzy/Knox]
**Level:** [XX]

### In-Game Statistics
- **Max Stage:** [Your stage]
- **Total Loot (Common):** [Number]
- **Total Loot (Uncommon):** [Number]
- **Total Loot (Rare):** [Number]
- **Total XP:** [Number]
- **Time to reach this stage:** [Approximate, e.g., 2 weeks]

### Build Details

#### Talents
[List all non-zero talents and their levels]
- Talent A: Level 5
- Talent B: Level 3
- etc.

#### Attributes
[Or attach screenshot showing full attribute distribution]
- Strength: 50
- Defense: 30
- etc.

#### Relics
- R1 (Relic Name): Level 3
- R7 (Manifestation Core): Level 5
- etc.

#### Inscryptions
- I33: Level 2
- I36: Level 1
- etc.

#### Gadgets & Gems
- Anchor: Level 2
- Attraction Loot: Level 3
- etc.

#### Global Account Bonuses
- Scavenger: Level 10
- Diamond Loot: Level 3
- Ultimate Multiplier: 1.5x
- etc.

### Screenshots
[Attach screenshots of hunter stats if possible]
```

### Option 2: JSON File

1. Use our export tool:
   ```bash
   cd hunter-sim
   python -c "
   from hunter_sim.gui_multi import export_build
   export_build('borge', 'my_borge_build.json')
   "
   ```

2. Attach `my_borge_build.json` to GitHub issue

### Option 3: Discord / Community

If you're in the CIFI community discord, you can also post your build there and tag the project maintainer!

## After You Submit

Here's what we do:

1. **Receive your submission** ✓
2. **Load your build** - Import into our validation system
3. **Run 50 simulations** - Compare avg stage vs your IRL stage
4. **Validate accuracy** - Check if our formula predicts correctly
5. **Report results** - Post validation report in the issue

### Example Report

```
✅ VALIDATION REPORT: Borge Level 69

IRL Build Performance:
  Stage: 300 (your max)
  
Simulation Results (50 runs):
  Avg Stage: 300.0 ± 0.5
  Min: 299
  Max: 301
  
Accuracy: ✅ PERFECT (0% deviation)

Detailed Stats Comparison:
  Metric          IRL          Simulated    Accuracy
  ─────────────────────────────────────────────────────
  Common Loot     373.8T       411.0T       +10% (acceptable)
  Uncommon Loot   352.9T       353.3T       +0.1% (perfect)
  Rare Loot       265.6T       266.0T       +0.1% (perfect)
  XP              7860T        5726T        -27% (needs investigation)

Status: ✅ ACCEPTED - Your build validates our stage prediction!
```

## Build Quality Guidelines

### What Makes a Good Submission

✅ **Good:**
- Active player (stage reached recently, not months old)
- Build fully optimized (you've spread all points)
- Screenshots provided (helps verify accuracy)
- Detailed talent/attribute breakdown
- Account bonuses documented

⚠️ **Acceptable:**
- Early-stage hunters (still good data, helps find mechanics)
- Partial build information (we can fill in blanks)
- Estimate of stats (if you don't have exact numbers)

❌ **Not Useful:**
- Abandoned builds (not actively played)
- Incomplete point allocation
- No verification possible
- Conflicting information between multiple submissions

## Privacy & Data

### What We Do With Your Data

- ✅ Use to validate formulas
- ✅ Publish anonymized results (e.g., "Knox L30, Stage 100")
- ✅ Share with community (to show accuracy)
- ❌ Never sell or share personally
- ❌ Not used outside this project

### Your Privacy

- Your GitHub username is public (if you use issues)
- Build details are publicly visible
- You can request anonymization: comment in your issue

## Frequently Asked Questions

**Q: Will this affect my game progress?**
A: No! We only read and analyze your build - we never modify your game files.

**Q: Can I submit multiple builds?**
A: Yes! Multiple builds from same player help us understand build variety.

**Q: What if my build doesn't validate?**
A: Great! That helps us find bugs. We'll investigate and report findings.

**Q: Can I update my submission?**
A: Yes, comment on your issue with updated stats/build info.

**Q: How long until my data is used?**
A: Usually within a few days. We batch-validate submissions.

**Q: Will I get credit for my submission?**
A: Yes! Contributors are listed in the README and release notes.

## Example Submissions (Templates)

### Template 1: Knox Early Stage

```markdown
[IRL-BUILD] Knox - Level 15

## In-Game Statistics
- **Max Stage:** 50
- **Total Loot (Common):** 5.2M
- **Total Loot (Uncommon):** 4.1M
- **Total Loot (Rare):** 3.2M
- **Total XP:** 1.2M

## Build Details

Talents (16 points):
- Unfair Advantage: 2
- Momentum: 3
- Guardian Angel: 2
- etc.

Attributes (45 points):
- Power Knox: 15
- Defense Knox: 10
- etc.

[Rest of template...]
```

### Template 2: Borge Mid-Game

```markdown
[IRL-BUILD] Borge - Level 50

## In-Game Statistics
- **Max Stage:** 200
- **Total Loot:** 45.3T (all types)
- **Total XP:** 892T
- **Time to reach:** 3 weeks of active play

[Detailed build breakdown]
[Screenshots attached]
```

### Template 3: Ozzy Late Game ⭐ PRIORITY

```markdown
[IRL-BUILD] Ozzy - Level 60+

**Priority:** HIGH - Ozzy needs validation!

[Full build details with screenshots]
```

## Need Help?

- **Questions about submission format?** - Comment on GitHub issue
- **Technical problems exporting build?** - Open bug report
- **Not sure about your build setup?** - Post to community discord first
- **General feedback?** - GitHub Discussions

## Leaderboard (Coming Soon!)

We're planning to feature validated builds in a community leaderboard:

```
Top Knox Builds (Validated)
══════════════════════════════════
1. Stage 105 - @Player1 (L35)
2. Stage 103 - @Player2 (L33)
3. Stage 101 - @Player3 (L31)

Top Borge Builds (Validated)
══════════════════════════════════
1. Stage 305 - @Player4 (L70)
2. Stage 304 - @Player5 (L69)
3. Stage 303 - @Player6 (L68)
```

**Your build could be featured here!** Submit your IRL build to get validated and potentially ranked.

## Thank You! 🙏

Every submission helps make this tool better for everyone. The CIFI community is built on shared knowledge - your build data is invaluable!

### We're especially looking for:
- 🔴 **Ozzy builds** (we need 10+ for proper validation)
- 🟡 **Early stage hunters** (helps find mechanics)
- 🟢 **Late game hunters** (validates high-stage formulas)
- 🔵 **Knox builds** (helps confirm our current accuracy)

**Together, we make the simulator better for the whole community!**

---

**Questions?** Open a GitHub issue or join the community discussion!
