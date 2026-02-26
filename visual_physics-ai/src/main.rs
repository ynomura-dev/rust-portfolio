use macroquad::prelude::*;

// 1. ボールの状態を表す構造体
struct Ball {
    x: f32,      // X座標（横）
    y: f32,      // Y座標（縦）
    velocity_y: f32, // 縦方向の速度
    radius: f32, // 半径
}

// 2. Macroquadのおまじない（非同期メイン関数）
#[macroquad::main("Physics Visualization")]
async fn main() {
    // 3. 初期設定
    let mut ball = Ball {
        x: screen_width() / 2.0, // 画面の中央
        y: 100.0,                // 上から100ピクセルの位置
        velocity_y: 0.0,         // 最初は止まっている
        radius: 20.0,            // 半径20ピクセル
    };

    // パラメータ設定（単位は ピクセル/秒 です）
    let gravity = 800.0; // 重力加速度（画面用なので大きめに設定）
    let bounce_damping = 0.8; // 跳ね返り係数（1.0で完全弾性衝突、小さいとすぐ止まる）

    // 4. ゲームループ（毎フレーム実行される無限ループ）
    loop {
        // --- 物理計算フェーズ (Update) ---

        // 前のフレームからの経過時間(Δt)を取得（これで動きが滑らかになる）
        let dt = get_frame_time();

        // 速度の更新 (v = v0 + at)
        ball.velocity_y += gravity * dt;

        // 位置の更新 (y = y0 + vt)
        ball.y += ball.velocity_y * dt;

        // 床との衝突判定
        // 画面の下端は screen_height() で取得できます
        if ball.y > screen_height() - ball.radius {
            // めり込み補正（ボールを床の表面に戻す）
            ball.y = screen_height() - ball.radius;
            // 速度を反転させて少し弱める（跳ね返り）
            ball.velocity_y = -ball.velocity_y * bounce_damping;
        }

        // --- 描画フェーズ (Draw) ---

        // 1. 背景をクリア（これを忘れると前の絵が残像で残ります）
        clear_background(LIGHTGRAY);

        // 2. ボールを描画
        draw_circle(ball.x, ball.y, ball.radius, RED);

        // 3. 情報を表示（デバッグ用）
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 20.0, 30.0, DARKGRAY);
        draw_text(&format!("Speed: {:.1}", ball.velocity_y), 20.0, 50.0, 20.0, BLACK);

        // 4. 次のフレームまで待機（必須！）
        next_frame().await
    }
}