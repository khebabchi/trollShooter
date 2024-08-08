package git.hassamamar.trollshooter.user;

import git.hassamamar.trollshooter.achievement.AchievementRepository;
import org.springframework.web.bind.annotation.*;

import java.util.List;
import java.util.Optional;

@RestController
@RequestMapping("/users")
public class UserController {
    UserRepository userRepository;
    AchievementRepository achievementRepository;

    public UserController(UserRepository userRepository, AchievementRepository achievementRepository) {
        this.userRepository = userRepository;
        this.achievementRepository = achievementRepository;
    }

    @PostMapping("")
    public void post(@RequestBody User user) {
        userRepository.insert(user.username(), user.email(), user.password());

    }

    @GetMapping("")
    public List<UserInfo> getAll() {
        return userRepository.getAllInfo();
    }

    @GetMapping("/{username}")
    public Optional<User> get(@PathVariable String username) {
        return userRepository.get(username);
    }
}
