# Model report for file:///tmp/top-repos-quality-repos-gqx56fkx/flask-debugtoolbar.git HEAD d474a6a689be916d65c2adf173e6517290902abe

### Dump

```json
{'created_at': '2021-08-29 13:10:30',
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
 'size': '19.1 kB',
 'tags': [],
 'uuid': '17a983af-f2a0-4bea-abc1-b58e689fbfb9',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-gqx56fkx/flask-debugtoolbar.git d474a6a689be916d65c2adf173e6517290902abe

# javascript
28 rules, avg.len. 7.6
## train
PPCR: 0.899170
### report
macro
{'f1-score': 0.45134032672059526,
 'precision': 0.4479553689837383,
 'recall': 0.4568962349597102,
 'support': 37909}
micro
{'f1-score': 0.9644411617294046,
 'precision': 0.9644411617294046,
 'recall': 0.9644411617294046,
 'support': 37909}
weighted
{'f1-score': 0.9580362436587129,
 'precision': 0.9525114920664178,
 'recall': 0.9644411617294046,
 'support': 37909}
### report_full
macro
{'f1-score': 0.4109063715948917,
 'precision': 0.4479553689837383,
 'recall': 0.38575819890099,
 'support': 42160}
micro
{'f1-score': 0.9132373328004596,
 'precision': 0.9644411617294046,
 'recall': 0.867196394686907,
 'support': 42160}
weighted
{'f1-score': 0.8933785727421575,
 'precision': 0.9235442191942004,
 'recall': 0.867196394686907,
 'support': 42160}
## test
PPCR: 0.901629
### report
macro
{'f1-score': 0.43314429330670867,
 'precision': 0.43924491461888715,
 'recall': 0.43752816500099734,
 'support': 1494}
micro
{'f1-score': 0.9471218206157965,
 'precision': 0.9471218206157965,
 'recall': 0.9471218206157965,
 'support': 1494}
weighted
{'f1-score': 0.9374419463793509,
 'precision': 0.9302625359892028,
 'recall': 0.9471218206157965,
 'support': 1494}
### report_full
macro
{'f1-score': 0.38332327126646765,
 'precision': 0.43924491461888715,
 'recall': 0.36182259356777835,
 'support': 1657}
micro
{'f1-score': 0.8981275785464933,
 'precision': 0.9471218206157965,
 'recall': 0.8539529269764635,
 'support': 1657}
weighted
{'f1-score': 0.8759299298934118,
 'precision': 0.9071882932868716,
 'recall': 0.8539529269764635,
 'support': 1657}
```

## javascript
### Summary
16 rules, avg.len. 8.2

| | |
|-|-|
|Min support|104|
|Max support|5947|
|Min confidence|0.9330986142158508|
|Max confidence|0.9993288516998291|

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
                     'min_samples_leaf': 91,
                     'min_samples_split': 181,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.997. Support: 5947.` |
| 2 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -3.reserved not in {=}<br>	∧ -5.roles not in {BINARY}<br>	∧ +1.reserved not in {)}<br>	∧ +2.length ≥ 3<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 1238.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved not in {=}<br>	∧ -5.roles not in {BINARY}<br>	∧ +1.reserved not in {)}<br>	∧ +2.length ≤ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 2074.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved not in {=}<br>	∧ -5.roles not in {BINARY}<br>	∧ +1.reserved not in {)}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ +2.length ≤ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 379.` |
| 5 | `  -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 745.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 5695.` |
| 7 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 2400.` |
| 8 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 1110.` |
| 9 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.reserved not in {;}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 142.` |
| 10 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 369.` |
| 11 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 141.` |
| 12 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 293.` |
| 13 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 144.` |
| 14 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved = (<br>	∧ -4.diff_col ≤ 6<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 104.` |
| 15 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ +1.roles in {NAME} and not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 175.` |
| 16 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 5711.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.25, "max_conf": 0.9993288516998291, "max_support": 5947, "min_conf": 0.9330986142158508, "min_support": 104, "num_rules": 16}}
```
</details>
