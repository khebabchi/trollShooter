package git.hassamamar.trollshooter.user;

import git.hassamamar.trollshooter.achievement.AchievementRepository;
import org.springframework.web.bind.annotation.*;

import java.util.List;
import java.util.Optional;

@RestController
@RequestMapping("")
public class UserController {
    UserRepository userRepository;
    AchievementRepository achievementRepository;

    public UserController(UserRepository userRepository, AchievementRepository achievementRepository) {
        this.userRepository = userRepository;
        this.achievementRepository = achievementRepository;
    }

    @PostMapping("/user/signin")
    public void post(@RequestBody User user) {
        userRepository.insert(user.username(), user.email(), user.password());
    }
    
    @PostMapping("/users/{username}/score/{score}")
    public void post(@PathVariable int score, @PathVariable String username) {
        userRepository.updateScore(username, score);
    }
    
    @GetMapping("/users")
    public List<UserInfo> getAll() {
        return userRepository.getAllInfo();
    }

    public record LoginInfo(
            String username,
            String password
    ) {
    }

    @PostMapping("/user/login")
    public Optional<User> get(@RequestBody LoginInfo loginInfo) {
        return userRepository.get(loginInfo);
    }
}
