# 実装計画

- [ ] 1. データモデルの更新

  - [ ] 1.1 Repository 構造体を更新（remote → url、local_path 削除）
    - `src/models/repository.rs` を修正
    - `name: String`, `url: String` のみに変更
    - _Requirements: 1.1_
  - [ ] 1.2 ManagedWorkspace 構造体を新規作成
    - `src/models/workspace.rs` に追加
    - `id`, `name`, `workspace_file`, `repos`, `created_at`, `updated_at` フィールド
    - _Requirements: 12.1, 12.2, 12.3, 12.4_
  - [ ] 1.3 Config 構造体を更新
    - `src/models/config.rs` を修正
    - `workspaces: Vec<ManagedWorkspace>` フィールドを追加
    - _Requirements: 10.1, 12.4_
  - [ ]\* 1.4 プロパティテスト: Config のシリアライズ/デシリアライズ ラウンドトリップ
    - **Property 13: Workspace JSON の有効性**
    - **Validates: Requirements 9.1, 9.3**

- [ ] 2. GitCloneManager の実装

  - [ ] 2.1 GitCloneManager trait を定義
    - `src/infrastructure/git/clone_manager.rs` を新規作成
    - `clone()`, `checkout_branch()`, `list_remote_branches()`, `set_upstream()` メソッド
    - _Requirements: 5.1, 5.2, 5.3, 7.1_
  - [ ] 2.2 DefaultGitCloneManager を実装
    - git2 クレートを使用した実装
    - _Requirements: 5.1, 5.2, 5.3_
  - [ ] 2.3 list_remote_branches の実装
    - git ls-remote 相当の処理を git2 で実装
    - デフォルトブランチ（main/master）を先頭にソート
    - _Requirements: 7.1, 7.2_
  - [ ]\* 2.4 プロパティテスト: ブランチソートの正確性
    - **Property 11: ブランチソートの正確性**
    - **Validates: Requirements 7.2**

- [ ] 3. チェックポイント - テスト確認

  - Ensure all tests pass, ask the user if questions arise.

- [ ] 4. RepositoryService の更新

  - [ ] 4.1 register メソッドを更新（bare clone を削除）
    - `src/services/repository.rs` を修正
    - URL 検証と config.json への保存のみ
    - _Requirements: 1.1, 1.2, 4.1, 4.2_
  - [ ] 4.2 unregister メソッドを更新（bare リポジトリ削除を削除）
    - config.json からのエントリ削除のみ
    - _Requirements: 3.1, 3.2_
  - [ ]\* 4.3 プロパティテスト: Register/Unregister ラウンドトリップ
    - **Property 1: Register/Unregister ラウンドトリップ**
    - **Validates: Requirements 1.1, 3.1**
  - [ ]\* 4.4 プロパティテスト: URL 検証の正確性
    - **Property 4: URL 検証の正確性**
    - **Validates: Requirements 4.1, 4.2**
  - [ ]\* 4.5 プロパティテスト: リポジトリ名抽出の一貫性
    - **Property 5: リポジトリ名抽出の一貫性**
    - **Validates: Requirements 4.3**

- [ ] 5. WorkspaceService の更新

  - [ ] 5.1 create_workspace メソッドを更新（worktree → clone）
    - `src/services/workspace.rs` を修正
    - GitCloneManager を使用した clone 処理
    - ManagedWorkspace の作成と config.json への保存
    - _Requirements: 5.1, 5.2, 5.3, 6.1, 6.2, 12.1, 12.2, 12.3, 12.4_
  - [ ] 5.2 generate_workspace_file メソッドを更新
    - workspace ファイル生成ロジック
    - _Requirements: 6.1, 6.2, 6.3, 6.4, 9.1, 9.2, 9.3_
  - [ ]\* 5.3 プロパティテスト: Workspace ファイルの完全性
    - **Property 8: Workspace ファイルの完全性**
    - **Validates: Requirements 6.2, 9.2**
  - [ ]\* 5.4 プロパティテスト: Workspace ファイル名の正確性
    - **Property 9: Workspace ファイル名の正確性**
    - **Validates: Requirements 6.3**

- [ ] 6. チェックポイント - テスト確認

  - Ensure all tests pass, ask the user if questions arise.

- [ ] 7. Workspace 管理機能の実装

  - [ ] 7.1 list_workspaces メソッドを実装
    - config.json から workspace 一覧を取得
    - _Requirements: 10.1, 10.2, 10.3_
  - [ ] 7.2 remove_workspace メソッドを実装
    - workspace ファイルと clone ディレクトリの削除
    - config.json からのエントリ削除
    - _Requirements: 11.2, 11.3, 11.4, 11.5_
  - [ ]\* 7.3 プロパティテスト: Workspace 削除の完全性
    - **Property 15: Workspace 削除の完全性**
    - **Validates: Requirements 11.2, 11.3**
  - [ ]\* 7.4 プロパティテスト: Workspace ID の一意性
    - **Property 16: Workspace ID の一意性**
    - **Validates: Requirements 12.1**

- [ ] 8. CLI コマンドの更新

  - [ ] 8.1 register コマンドを更新
    - `src/commands/register.rs` を修正
    - bare clone 処理を削除
    - _Requirements: 1.1, 1.5_
  - [ ] 8.2 unregister コマンドを更新
    - `src/commands/unregister.rs` を修正
    - bare リポジトリ削除処理を削除
    - _Requirements: 3.1, 3.3_
  - [ ] 8.3 list-workspaces コマンドを新規作成
    - `src/commands/list_workspaces.rs` を新規作成
    - workspace 一覧表示
    - _Requirements: 10.1, 10.2, 10.3_
  - [ ] 8.4 rm-workspace コマンドを新規作成
    - `src/commands/rm_workspace.rs` を新規作成
    - インタラクティブな workspace 選択と削除
    - _Requirements: 11.1, 11.4, 11.5_
  - [ ] 8.5 CLI 定義を更新
    - `src/cli/mod.rs` に新コマンドを追加
    - _Requirements: 10.1, 11.1_

- [ ] 9. チェックポイント - テスト確認

  - Ensure all tests pass, ask the user if questions arise.

- [ ] 10. 不要コードの削除

  - [ ] 10.1 WorktreeManager を削除
    - `src/infrastructure/git/worktree.rs` を削除
    - 関連する import を削除
  - [ ] 10.2 bare_clone 関連コードを削除
    - `src/infrastructure/git/operations.rs` から bare_clone を削除
  - [ ] 10.3 テストヘルパーを更新
    - `src/utils/test_helpers.rs` から bare リポジトリ関連ヘルパーを削除

- [ ] 11. 最終チェックポイント - テスト確認
  - Ensure all tests pass, ask the user if questions arise.
