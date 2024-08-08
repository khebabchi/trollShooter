package git.hassamamar.trollshooter.user;

import org.springframework.jdbc.core.simple.JdbcClient;
import org.springframework.stereotype.Repository;
import org.springframework.util.Assert;

import java.time.LocalDate;
import java.util.List;
import java.util.Optional;

@Repository
public class UserRepository {
    private final JdbcClient jdbcClient;

    public UserRepository(JdbcClient jdbcClient) {
        this.jdbcClient = jdbcClient;
    }

    public void insert(String username, String email, String password) {
        int update = jdbcClient.sql("INSERT INTO users (username, email, password, top_score, created_at) " +
                        "VALUES (:username, :email, :password,0,:created_at);")
                .param("username", username).param("email", email).param("password", password).param("created_at", LocalDate.now()).update();
        Assert.state(update == 1, "Failed to create user " + username);
    }

    public List<UserInfo> getAllInfo() {
        return jdbcClient.sql("SELECT username,top_score FROM users ORDER BY top_score DESC LIMIT 6").query(UserInfo.class).list();
    }

    public Optional<User> get(String username) {
        return jdbcClient.sql("SELECT * FROM users WHERE username=:username").param("username", username).query(User.class).list().stream().findFirst();
    }
}

