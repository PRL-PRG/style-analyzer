# Model report for file:///tmp/top-repos-quality-repos-g29ub55i/idict-weapp.git HEAD 115e3390aa931e4955cdb4fc2ee5d7eb1eccde36

### Dump

```json
{'created_at': '2021-08-21 19:06:19',
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
 'size': '21.9 kB',
 'tags': [],
 'uuid': '3dca580f-d860-489a-ac95-dfc59986b064',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-g29ub55i/idict-weapp.git 115e3390aa931e4955cdb4fc2ee5d7eb1eccde36

# javascript
29 rules, avg.len. 8.8
## train
PPCR: 0.879667
### report
macro
{'f1-score': 0.5091046014650861,
 'precision': 0.51361382809923,
 'recall': 0.5079500946487108,
 'support': 28554}
micro
{'f1-score': 0.9443510541430272,
 'precision': 0.9443510541430272,
 'recall': 0.9443510541430272,
 'support': 28554}
weighted
{'f1-score': 0.9375288518569838,
 'precision': 0.9322358644827948,
 'recall': 0.9443510541430272,
 'support': 28554}
### report_full
macro
{'f1-score': 0.41775234134313116,
 'precision': 0.51361382809923,
 'recall': 0.3758747956478102,
 'support': 32460}
micro
{'f1-score': 0.8838954993935818,
 'precision': 0.9443510541430272,
 'recall': 0.8307147258163894,
 'support': 32460}
weighted
{'f1-score': 0.860592759324781,
 'precision': 0.9143478251042378,
 'recall': 0.8307147258163894,
 'support': 32460}
## test
PPCR: 0.868504
### report
macro
{'f1-score': 0.4590592296625707,
 'precision': 0.45521532886892224,
 'recall': 0.4718491299040615,
 'support': 7754}
micro
{'f1-score': 0.9352592210472015,
 'precision': 0.9352592210472015,
 'recall': 0.9352592210472015,
 'support': 7754}
weighted
{'f1-score': 0.9262680286292052,
 'precision': 0.91959093169074,
 'recall': 0.9352592210472015,
 'support': 7754}
### report_full
macro
{'f1-score': 0.35980226511123564,
 'precision': 0.45521532886892224,
 'recall': 0.3329323088032385,
 'support': 8928}
micro
{'f1-score': 0.8694401150941135,
 'precision': 0.9352592210472015,
 'recall': 0.8122759856630825,
 'support': 8928}
weighted
{'f1-score': 0.8411266242522636,
 'precision': 0.8990093750041569,
 'recall': 0.8122759856630825,
 'support': 8928}
```

## javascript
### Summary
14 rules, avg.len. 7.5

| | |
|-|-|
|Min support|98|
|Max support|5082|
|Min confidence|0.9338117837905884|
|Max confidence|0.9973683953285217|

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
| 1 | `  +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 186.` |
| 2 | `  -1.label not in {<space>}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 112.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {LITERAL}<br>	∧ +1.reserved not in {), ]}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.934. Support: 3135.` |
| 4 | `  ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 5082.` |
| 5 | `  -1.internal_type = Identifier<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 281.` |
| 6 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 3237.` |
| 7 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 1630.` |
| 8 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 1145.` |
| 9 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label in {<newline>}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 818.` |
| 10 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 99.` |
| 11 | `  •••start_col ≥ 16<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<newline>}<br>	∧ -3.diff_offset ≤ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {EXPRESSION, LITERAL} and not in {KEY}<br>	∧ ^1.internal_type not in {IfStatement, MemberExpression}<br>	∧ ^1.roles not in {BLOCK, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 98.` |
| 12 | `  -1.internal_type not in {Identifier}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {EXPRESSION} and not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 264.` |
| 13 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {BLOCK} and not in {STRING}<br>	∧ +1.reserved not in {(, ;, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles in {EXPRESSION} and not in {COMMENT}<br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 190.` |
| 14 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {(, ;, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT, EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 1250.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.5, "max_conf": 0.9973683953285217, "max_support": 5082, "min_conf": 0.9338117837905884, "min_support": 98, "num_rules": 14}}
```
</details>
