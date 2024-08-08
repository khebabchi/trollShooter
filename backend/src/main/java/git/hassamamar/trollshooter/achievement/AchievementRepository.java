package git.hassamamar.trollshooter.achievement;

import org.springframework.jdbc.core.simple.JdbcClient;
import org.springframework.stereotype.Repository;
import org.springframework.util.Assert;

import java.time.LocalDate;
import java.util.List;


@Repository
public class AchievementRepository {
    private final JdbcClient jdbcClient;

    public AchievementRepository(JdbcClient jdbcClient) {
        this.jdbcClient = jdbcClient;
    }


    public void insert(int id, String username) {
        int update = jdbcClient.sql("INSERT INTO achievements (id,username,unlocked_at) VALUES (:id,:username,:unlocked_at);")
                .param("id", id)
                .param("username", username)
                .param("unlocked_at", LocalDate.now())
                .update();
        Assert.state(update == 1, "Failed to create achievement" + id + " for " + username);
    }


    public List<Achievement> getAll(String username) {
        System.out.println(username);
        return jdbcClient.sql("SELECT * FROM achievements WHERE username=:username;").param("username", username).query(Achievement.class).list();
    }
}
