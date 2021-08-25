# Model report for file:///tmp/top-repos-quality-repos-9_onuwcr/sixcfj.git HEAD 38e495aabf5986ba91fc34fc3708a69a22cf92b7

### Dump

```json
{'created_at': '2021-08-20 23:03:16',
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
 'size': '16.7 kB',
 'tags': [],
 'uuid': 'd8e6de9c-e197-4aa8-a1ea-57a54ee375e5',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-9_onuwcr/sixcfj.git 38e495aabf5986ba91fc34fc3708a69a22cf92b7

# javascript
20 rules, avg.len. 7.8
## train
PPCR: 0.788216
### report
macro
{'f1-score': 0.4751307568157444,
 'precision': 0.5295184732905535,
 'recall': 0.44941365622894786,
 'support': 10261}
micro
{'f1-score': 0.8661923789104377,
 'precision': 0.8661923789104375,
 'recall': 0.8661923789104375,
 'support': 10261}
weighted
{'f1-score': 0.8407228754963345,
 'precision': 0.840443743173774,
 'recall': 0.8661923789104375,
 'support': 10261}
### report_full
macro
{'f1-score': 0.3772425805408396,
 'precision': 0.5295184732905535,
 'recall': 0.32302172657758144,
 'support': 13018}
micro
{'f1-score': 0.7636066841359165,
 'precision': 0.8661923789104375,
 'recall': 0.6827469657397449,
 'support': 13018}
weighted
{'f1-score': 0.7096244897628222,
 'precision': 0.8258612774541503,
 'recall': 0.6827469657397449,
 'support': 13018}
## test
PPCR: 0.735931
### report
macro
{'f1-score': 0.4453994471843971,
 'precision': 0.5991608574767296,
 'recall': 0.41791687494717233,
 'support': 1530}
micro
{'f1-score': 0.8627450980392157,
 'precision': 0.8627450980392157,
 'recall': 0.8627450980392157,
 'support': 1530}
weighted
{'f1-score': 0.8387170588059271,
 'precision': 0.8693408561124951,
 'recall': 0.8627450980392157,
 'support': 1530}
### report_full
macro
{'f1-score': 0.3303431262924214,
 'precision': 0.5991608574767296,
 'recall': 0.2717247034803014,
 'support': 2079}
micro
{'f1-score': 0.7315045719035744,
 'precision': 0.8627450980392157,
 'recall': 0.6349206349206349,
 'support': 2079}
weighted
{'f1-score': 0.6749097692847185,
 'precision': 0.8570669770250703,
 'recall': 0.6349206349206349,
 'support': 2079}
```

## javascript
### Summary
6 rules, avg.len. 8.3

| | |
|-|-|
|Min support|137|
|Max support|5601|
|Min confidence|0.9218889474868774|
|Max confidence|0.9946351647377014|

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
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
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
| 1 | `  -1.reserved = :<br>⇒ y = ␣<br>Confidence: 0.995. Support: 466.` |
| 2 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, :}<br>⇒ y = ⏎<br>Confidence: 0.929. Support: 261.` |
| 3 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {STATEMENT} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 0.926. Support: 168.` |
| 4 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 5601.` |
| 5 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {AssignmentExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 164.` |
| 6 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 137.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.333333333333334, "max_conf": 0.9946351647377014, "max_support": 5601, "min_conf": 0.9218889474868774, "min_support": 137, "num_rules": 6}}
```
</details>
