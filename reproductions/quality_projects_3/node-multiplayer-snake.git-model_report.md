# Model report for file:///tmp/top-repos-quality-repos-mg30wq4w/node-multiplayer-snake.git HEAD c21d4a49f1b8dec6f0a2005dfa97ac50c465d81f

### Dump

```json
{'created_at': '2021-08-29 09:39:17',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.19.0-12-amd64-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.5 kB',
 'tags': [],
 'uuid': 'e0b847f9-0c30-4aba-bdbc-c3589eb71935',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-mg30wq4w/node-multiplayer-snake.git c21d4a49f1b8dec6f0a2005dfa97ac50c465d81f

# javascript
16 rules, avg.len. 5.3
## train
PPCR: 0.870111
### report
macro
{'f1-score': 0.9172475660315544,
 'precision': 0.9432373274095186,
 'recall': 0.897273733775908,
 'support': 16506}
micro
{'f1-score': 0.966981703622925,
 'precision': 0.966981703622925,
 'recall': 0.966981703622925,
 'support': 16506}
weighted
{'f1-score': 0.9665281095433811,
 'precision': 0.9670364558709861,
 'recall': 0.966981703622925,
 'support': 16506}
### report_full
macro
{'f1-score': 0.7773084154701098,
 'precision': 0.9432373274095186,
 'recall': 0.7120474925722303,
 'support': 18970}
micro
{'f1-score': 0.8998195963468261,
 'precision': 0.966981703622925,
 'recall': 0.8413811280969953,
 'support': 18970}
weighted
{'f1-score': 0.8821471987599887,
 'precision': 0.9601087421355383,
 'recall': 0.8413811280969953,
 'support': 18970}
## test
PPCR: 0.884367
### report
macro
{'f1-score': 0.8409903206877986,
 'precision': 0.8565866061772837,
 'recall': 0.8441197340031998,
 'support': 4107}
micro
{'f1-score': 0.9719990260530801,
 'precision': 0.9719990260530801,
 'recall': 0.9719990260530801,
 'support': 4107}
weighted
{'f1-score': 0.975746650083863,
 'precision': 0.9808252288094196,
 'recall': 0.9719990260530801,
 'support': 4107}
### report_full
macro
{'f1-score': 0.7404147441353126,
 'precision': 0.8565866061772837,
 'recall': 0.6809292287446901,
 'support': 4644}
micro
{'f1-score': 0.912352873957262,
 'precision': 0.9719990260530801,
 'recall': 0.859603789836348,
 'support': 4644}
weighted
{'f1-score': 0.8932900865874888,
 'precision': 0.9472522529196352,
 'recall': 0.859603789836348,
 'support': 4644}
```

## javascript
### Summary
13 rules, avg.len. 5.2

| | |
|-|-|
|Min support|94|
|Max support|4642|
|Min confidence|0.9574536681175232|
|Max confidence|0.9992570877075195|

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
                     'max_depth': 10,
                     'min_samples_leaf': 90,
                     'min_samples_split': 185,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.996. Support: 3543.` |
| 2 | `  -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.997. Support: 163.` |
| 3 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 673.` |
| 4 | `  -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.982. Support: 428.` |
| 5 | `  -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<-space>}<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎⏎<br>Confidence: 0.996. Support: 133.` |
| 6 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {IDENTIFIER, KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.995. Support: 99.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.length ≤ 2<br>	∧ +1.roles not in {IDENTIFIER, KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {IF}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 94.` |
| 8 | `  +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 525.` |
| 9 | `  +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 455.` |
| 10 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.998. Support: 274.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 174.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 109.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 4642.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.230769230769231, "max_conf": 0.9992570877075195, "max_support": 4642, "min_conf": 0.9574536681175232, "min_support": 94, "num_rules": 13}}
```
</details>
