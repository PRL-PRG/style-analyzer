# Model report for file:///tmp/top-repos-quality-repos-h2llj9ax/bench.git HEAD 363e374742ed1f4a994de6ca86a1f9bce03986a8

### Dump

```json
{'created_at': '2021-08-21 11:54:17',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '18.6 kB',
 'tags': [],
 'uuid': '45516b80-6727-47a2-98b4-76462397d15c',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-h2llj9ax/bench.git 363e374742ed1f4a994de6ca86a1f9bce03986a8

# javascript
12 rules, avg.len. 6.2
## train
PPCR: 0.738939
### report
macro
{'f1-score': 0.34167812814701204,
 'precision': 0.37067481017607673,
 'recall': 0.32598146162479746,
 'support': 11090}
micro
{'f1-score': 0.9505861136158702,
 'precision': 0.9505861136158702,
 'recall': 0.9505861136158702,
 'support': 11090}
weighted
{'f1-score': 0.9447604866446178,
 'precision': 0.9424184067069153,
 'recall': 0.9505861136158702,
 'support': 11090}
### report_full
macro
{'f1-score': 0.2879426979651604,
 'precision': 0.37067481017607673,
 'recall': 0.2505485846848516,
 'support': 15008}
micro
{'f1-score': 0.8078779983140472,
 'precision': 0.9505861136158702,
 'recall': 0.7024253731343284,
 'support': 15008}
weighted
{'f1-score': 0.7366206443267874,
 'precision': 0.7895812001878632,
 'recall': 0.7024253731343284,
 'support': 15008}
## test
PPCR: 0.705762
### report
macro
{'f1-score': 0.31030905563888,
 'precision': 0.3512621773627449,
 'recall': 0.2938951208726127,
 'support': 2413}
micro
{'f1-score': 0.9129714048901783,
 'precision': 0.9129714048901783,
 'recall': 0.9129714048901783,
 'support': 2413}
weighted
{'f1-score': 0.8990402150931276,
 'precision': 0.8922273757348718,
 'recall': 0.9129714048901783,
 'support': 2413}
### report_full
macro
{'f1-score': 0.27267568752872423,
 'precision': 0.3512621773627449,
 'recall': 0.24079134794162305,
 'support': 3419}
micro
{'f1-score': 0.7554869684499315,
 'precision': 0.9129714048901783,
 'recall': 0.6443404504241006,
 'support': 3419}
weighted
{'f1-score': 0.665224726649755,
 'precision': 0.7039451790637107,
 'recall': 0.6443404504241006,
 'support': 3419}
```

## javascript
### Summary
9 rules, avg.len. 6.2

| | |
|-|-|
|Min support|100|
|Max support|2752|
|Min confidence|0.944915235042572|
|Max confidence|0.996874988079071|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 2070.` |
| 2 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 458.` |
| 3 | `  •••start_col ≤ 11<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.959. Support: 109.` |
| 4 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE} and not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.945. Support: 118.` |
| 5 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 160.` |
| 6 | `  +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 513.` |
| 7 | `  -1.internal_type = Identifier<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 100.` |
| 8 | `  -1.roles not in {STRING}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 358.` |
| 9 | `  -1.roles not in {STRING}<br>	∧ -3.diff_line = 0<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 2752.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.222222222222222, "max_conf": 0.996874988079071, "max_support": 2752, "min_conf": 0.944915235042572, "min_support": 100, "num_rules": 9}}
```
</details>
