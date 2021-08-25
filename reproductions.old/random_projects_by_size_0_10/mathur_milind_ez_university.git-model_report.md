# Model report for file:///tmp/top-repos-quality-repos-lf2e4na8/mathur_milind_ez_university.git HEAD 0bdd3ad78c2a34c4331314192556ed9137b30e5c

### Dump

```json
{'created_at': '2021-08-22 18:09:04',
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
 'size': '17.1 kB',
 'tags': [],
 'uuid': 'fa2183d6-30f7-4e9f-b77a-9ba71cacc758',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-lf2e4na8/mathur_milind_ez_university.git 0bdd3ad78c2a34c4331314192556ed9137b30e5c

# javascript
12 rules, avg.len. 6.1
## train
PPCR: 0.857066
### report
macro
{'f1-score': 0.6810024229705546,
 'precision': 0.7121030070630996,
 'recall': 0.6592753176860982,
 'support': 12718}
micro
{'f1-score': 0.9411070923101117,
 'precision': 0.9411070923101117,
 'recall': 0.9411070923101117,
 'support': 12718}
weighted
{'f1-score': 0.9327004684951932,
 'precision': 0.9267220084750756,
 'recall': 0.9411070923101117,
 'support': 12718}
### report_full
macro
{'f1-score': 0.5883324820552581,
 'precision': 0.7121030070630996,
 'recall': 0.5348431983368424,
 'support': 14839}
micro
{'f1-score': 0.8686722066988424,
 'precision': 0.9411070923101117,
 'recall': 0.8065907406159445,
 'support': 14839}
weighted
{'f1-score': 0.8452235360048697,
 'precision': 0.9180808448579318,
 'recall': 0.8065907406159445,
 'support': 14839}
## test
PPCR: 0.839107
### report
macro
{'f1-score': 0.6351104910723162,
 'precision': 0.6437842375905802,
 'recall': 0.6477100459105315,
 'support': 2519}
micro
{'f1-score': 0.8868598650258039,
 'precision': 0.8868598650258039,
 'recall': 0.8868598650258039,
 'support': 2519}
weighted
{'f1-score': 0.8802582447162834,
 'precision': 0.8798419680500502,
 'recall': 0.8868598650258039,
 'support': 2519}
### report_full
macro
{'f1-score': 0.5158601829227809,
 'precision': 0.6437842375905802,
 'recall': 0.46369310858234525,
 'support': 3002}
micro
{'f1-score': 0.8092736823039306,
 'precision': 0.8868598650258039,
 'recall': 0.7441705529646903,
 'support': 3002}
weighted
{'f1-score': 0.785248792621696,
 'precision': 0.8618676698012192,
 'recall': 0.7441705529646903,
 'support': 3002}
```

## javascript
### Summary
8 rules, avg.len. 6.4

| | |
|-|-|
|Min support|161|
|Max support|3688|
|Min confidence|0.9409937858581543|
|Max confidence|0.9973261952400208|

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
                     'min_samples_leaf': 120,
                     'min_samples_split': 182,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.label in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.944. Support: 905.` |
| 2 | `  -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.977. Support: 194.` |
| 3 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 399.` |
| 4 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.973. Support: 241.` |
| 5 | `  -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 257.` |
| 6 | `  -1.reserved = var<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 187.` |
| 7 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, ;, var, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.941. Support: 161.` |
| 8 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;, var, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ -4.diff_line = 0<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 3688.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.375, "max_conf": 0.9973261952400208, "max_support": 3688, "min_conf": 0.9409937858581543, "min_support": 161, "num_rules": 8}}
```
</details>
