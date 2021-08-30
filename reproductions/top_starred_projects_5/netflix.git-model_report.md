# Model report for file:///tmp/top-repos-quality-repos-axjjbbuo/netflix.git HEAD 54ead17f6b8a240b4ab0d2eeff9d1c66c0a6c206

### Dump

```json
{'created_at': '2021-08-30 00:28:20',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.9 kB',
 'tags': [],
 'uuid': '69cfec0a-a528-4172-84c8-eb55d408dd45',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-axjjbbuo/netflix.git 54ead17f6b8a240b4ab0d2eeff9d1c66c0a6c206

# javascript
18 rules, avg.len. 6.6
## train
PPCR: 0.785563
### report
macro
{'f1-score': 0.6939136088513653,
 'precision': 0.7193550668874041,
 'recall': 0.6757908492734899,
 'support': 11895}
micro
{'f1-score': 0.9508196721311475,
 'precision': 0.9508196721311475,
 'recall': 0.9508196721311475,
 'support': 11895}
weighted
{'f1-score': 0.9455524646974774,
 'precision': 0.9414875255849405,
 'recall': 0.9508196721311475,
 'support': 11895}
### report_full
macro
{'f1-score': 0.6119093606323542,
 'precision': 0.7193550668874041,
 'recall': 0.5506535633154537,
 'support': 15142}
micro
{'f1-score': 0.8366312830565521,
 'precision': 0.9508196721311475,
 'recall': 0.7469290714568749,
 'support': 15142}
weighted
{'f1-score': 0.8134190200943454,
 'precision': 0.9042424578778656,
 'recall': 0.7469290714568749,
 'support': 15142}
## test
PPCR: 0.747621
### report
macro
{'f1-score': 0.5961065090215166,
 'precision': 0.7099797442078479,
 'recall': 0.5658723906311016,
 'support': 2121}
micro
{'f1-score': 0.9311645450259312,
 'precision': 0.9311645450259312,
 'recall': 0.9311645450259312,
 'support': 2121}
weighted
{'f1-score': 0.917579754156554,
 'precision': 0.9149889752219209,
 'recall': 0.9311645450259312,
 'support': 2121}
### report_full
macro
{'f1-score': 0.42925106768983545,
 'precision': 0.7099797442078479,
 'recall': 0.3759105873715528,
 'support': 2837}
micro
{'f1-score': 0.7966922146026624,
 'precision': 0.9311645450259312,
 'recall': 0.6961579132886853,
 'support': 2837}
weighted
{'f1-score': 0.734328991685328,
 'precision': 0.8143253910202186,
 'recall': 0.6961579132886853,
 'support': 2837}
```

## javascript
### Summary
13 rules, avg.len. 7.3

| | |
|-|-|
|Min support|90|
|Max support|5935|
|Min confidence|0.9536585211753845|
|Max confidence|0.9966443181037903|

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
                     'min_samples_split': 190,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.label in {<space>}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.997. Support: 149.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.985. Support: 368.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.995. Support: 109.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ +4.reserved = =<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎⏎<br>Confidence: 0.969. Support: 112.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.972. Support: 90.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {CALL} and not in {DECLARATION}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.975. Support: 142.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {CALL, DECLARATION}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.995. Support: 98.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {CALL, DECLARATION}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 107.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 205.` |
| 10 | `  •••start_line ≥ 8<br>	∧ -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 179.` |
| 11 | `  •••start_line ≥ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>	∧ ^2.roles in {MAP}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 124.` |
| 12 | `  •••start_col ≥ 22<br>	∧ •••start_line ≥ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.label not in {<newline>}<br>	∧ -4.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 143.` |
| 13 | `  •••start_line ≥ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.label not in {<newline>}<br>	∧ -4.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 5935.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.3076923076923075, "max_conf": 0.9966443181037903, "max_support": 5935, "min_conf": 0.9536585211753845, "min_support": 90, "num_rules": 13}}
```
</details>
