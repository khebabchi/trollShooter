package git.hassamamar.trollshooter.achievement;

import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/users/{userId}/achievements")
public class AchievementController {
    AchievementRepository achievementRepository;

    public AchievementController(AchievementRepository achievementRepository) {
        this.achievementRepository = achievementRepository;
    }

    @PostMapping("/{id}")
    public void post(@PathVariable String userId, @PathVariable int id) {
        achievementRepository.insert(id, userId);
    }

    @GetMapping("")
    public List<Achievement> get(@PathVariable String userId) {
        return achievementRepository.getAll(userId);
    }
}
