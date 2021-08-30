# Model report for file:///tmp/top-repos-quality-repos-1flcresv/flexslider.git HEAD 690832b7f972298e76e2965714657a2beec9e35c

### Dump

```json
{'created_at': '2021-08-30 02:34:38',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.11.0-31-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '17.7 kB',
 'tags': [],
 'uuid': 'c930e16d-3ad1-41f2-98d2-889e2e47035b',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-1flcresv/flexslider.git 690832b7f972298e76e2965714657a2beec9e35c

# javascript
17 rules, avg.len. 6.8
## train
PPCR: 0.842533
### report
macro
{'f1-score': 0.39768786311599036,
 'precision': 0.39823932974284965,
 'recall': 0.3974038237976797,
 'support': 17614}
micro
{'f1-score': 0.9482230044282957,
 'precision': 0.9482230044282957,
 'recall': 0.9482230044282957,
 'support': 17614}
weighted
{'f1-score': 0.9407126125322445,
 'precision': 0.9336382114248128,
 'recall': 0.9482230044282957,
 'support': 17614}
### report_full
macro
{'f1-score': 0.31297150301040005,
 'precision': 0.39823932974284965,
 'recall': 0.27781346200272705,
 'support': 20906}
micro
{'f1-score': 0.8671858774662512,
 'precision': 0.9482230044282957,
 'recall': 0.798909403998852,
 'support': 20906}
weighted
{'f1-score': 0.8209995564356058,
 'precision': 0.8633167368860825,
 'recall': 0.798909403998852,
 'support': 20906}
## test
PPCR: 0.881690
### report
macro
{'f1-score': 0.21593726877619776,
 'precision': 0.2675343991996964,
 'recall': 0.21802152633010083,
 'support': 2817}
micro
{'f1-score': 0.7284345047923322,
 'precision': 0.7284345047923323,
 'recall': 0.7284345047923323,
 'support': 2817}
weighted
{'f1-score': 0.732356394902378,
 'precision': 0.790551787530583,
 'recall': 0.7284345047923323,
 'support': 2817}
### report_full
macro
{'f1-score': 0.18392851811379116,
 'precision': 0.2675343991996964,
 'recall': 0.17523461502715362,
 'support': 3195}
micro
{'f1-score': 0.6826347305389222,
 'precision': 0.7284345047923323,
 'recall': 0.6422535211267606,
 'support': 3195}
weighted
{'f1-score': 0.6692316981346221,
 'precision': 0.7462171155661437,
 'recall': 0.6422535211267606,
 'support': 3195}
```

## javascript
### Summary
10 rules, avg.len. 5.3

| | |
|-|-|
|Min support|104|
|Max support|4340|
|Min confidence|0.9446969628334045|
|Max confidence|0.9983108043670654|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.981. Support: 4340.` |
| 2 | `  -1.reserved = (<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 156.` |
| 3 | `  -1.internal_type = Identifier<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 118.` |
| 4 | `  -1.internal_type = Identifier<br>	∧ +2.reserved = ?<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 104.` |
| 5 | `  -1.internal_type = Identifier<br>	∧ ^1.internal_type not in {ConditionalExpression, MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 1568.` |
| 6 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 1104.` |
| 7 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 660.` |
| 8 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 296.` |
| 9 | `  -1.internal_type not in {CommentLine, Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 171.` |
| 10 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine, Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {CALL} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 270.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.3, "max_conf": 0.9983108043670654, "max_support": 4340, "min_conf": 0.9446969628334045, "min_support": 104, "num_rules": 10}}
```
</details>
